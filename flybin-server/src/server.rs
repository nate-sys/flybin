use std::{collections::HashMap, fmt, str::FromStr, sync::Arc, time::Duration};

use serde::{de, Deserialize, Deserializer};
use tower::{buffer::BufferLayer, limit::RateLimitLayer, ServiceBuilder};

use axum::{
    error_handling::HandleErrorLayer,
    extract::{FromRef, MatchedPath, Path, Query, State},
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
    routing::{delete, get, post},
    BoxError, Router,
};
use flybin_common::paste::Paste;
use sqlx::SqlitePool;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, Theme, ThemeSet},
    html::{
        append_highlighted_html_for_styled_line, start_highlighted_html_snippet, IncludeBackground,
    },
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};

use tower_http::trace::TraceLayer;

use crate::error::AppError;

#[derive(Clone)]
pub struct AppState {
    pub pool: Arc<SqlitePool>,
    pub ps: SyntaxSet,
    pub theme: Theme,
}
impl AppState {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self {
            pool,
            ps: SyntaxSet::load_defaults_newlines(),
            theme: ThemeSet::load_defaults().themes["base16-ocean.dark"].to_owned(),
        }
    }
}

impl FromRef<AppState> for Arc<SqlitePool> {
    fn from_ref(state: &AppState) -> Arc<SqlitePool> {
        state.pool.clone()
    }
}

impl FromRef<AppState> for SyntaxSet {
    fn from_ref(state: &AppState) -> SyntaxSet {
        state.ps.to_owned()
    }
}

impl FromRef<AppState> for Theme {
    fn from_ref(state: &AppState) -> Theme {
        state.theme.clone()
    }
}

#[derive(Debug, Deserialize)]
struct AppParams {
    secret: Option<String>,
    #[serde(default, deserialize_with = "empty_string_as_none")]
    password: Option<String>,
}

fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

pub async fn run(pool: Arc<SqlitePool>) -> anyhow::Result<()> {
    let app_state = AppState::new(pool);
    let app = Router::new()
        .route("/:slug", get(get_paste))
        .route("/:slug", post(lock_paste))
        .route("/:slug", delete(delete_paste))
        .route("/:slug/:lang", get(get_highlighted_paste))
        .layer(
            ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|err: BoxError|async move{
                (StatusCode::INTERNAL_SERVER_ERROR, format!("{:?}", err))
            }))
            .layer(BufferLayer::new(1024))
            .layer(RateLimitLayer::new(5, Duration::from_secs(30)))
            .layer(
            TraceLayer::new_for_http().make_span_with(|request: &axum::http::Request<_>| {
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);
                tracing::info_span!("http_request", method = ?request.method(), matched_path, headers = ?request.headers())
            }))
        )
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[axum::debug_handler]
async fn get_paste(
    Path(slug): Path<String>,
    Query(params): Query<AppParams>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<impl IntoResponse, AppError> {
    Ok(Paste::get(slug, params.password, &pool)
        .await
        .map(|paste| {
            (
                StatusCode::OK,
                [(header::CONTENT_TYPE, "text/plain")],
                paste.content,
            )
        })?)
}

#[axum::debug_handler]
async fn get_highlighted_paste(
    Path((slug, lang)): Path<(String, String)>,
    State(app_state): State<AppState>,
    Query(params): Query<AppParams>,
    headers: HeaderMap,
) -> Result<impl IntoResponse, AppError> {
    Paste::get(slug, params.password, &app_state.pool)
        .await
        .map(|paste| {
            if let Some(syntax) = app_state.ps.find_syntax_by_token(&lang) {
                let mut h = HighlightLines::new(syntax, &app_state.theme);
                let mut highlighted_content = String::new();
                let content_type;
                if headers["User-Agent"].to_str().unwrap().contains("curl/") {
                    content_type = "text/plain";
                    for line in LinesWithEndings::from(&paste.content) {
                        let ranges: Vec<(Style, &str)> = h
                            .highlight_line(line, &app_state.ps)
                            .map_err(|_| AppError::internal())?;
                        let escaped = as_24_bit_terminal_escaped(&ranges[..], true);
                        highlighted_content.push_str(&escaped);
                    }
                } else {
                    content_type = "text/html";
                    let (mut output, bg) = start_highlighted_html_snippet(&app_state.theme);

                    for line in LinesWithEndings::from(&paste.content) {
                        let regions = h.highlight_line(line, &app_state.ps).unwrap();
                        append_highlighted_html_for_styled_line(
                            &regions[..],
                            IncludeBackground::IfDifferent(bg),
                            &mut output,
                        )
                        .map_err(|_| AppError::internal())?;
                    }
                    output.push_str("</pre>\n");
                    highlighted_content = output;
                }

                Ok((
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, content_type)],
                    highlighted_content,
                ))
            } else {
                Ok((
                    StatusCode::OK,
                    [(header::CONTENT_TYPE, "text/plain")],
                    paste.content,
                ))
            }
        })?
}

#[axum::debug_handler]
async fn lock_paste(
    Path(slug): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Query(params): Query<AppParams>,
) -> Result<impl IntoResponse, AppError> {
    let password = params.password.ok_or(AppError(
        StatusCode::BAD_REQUEST,
        "Missing password".to_string(),
    ))?;

    let secret = params.secret.ok_or(AppError(
        StatusCode::BAD_REQUEST,
        "Missing secret".to_string(),
    ))?;

    if Paste::lock(&slug, secret, password, &pool).await? == 0 {
        Err(AppError(
            StatusCode::NOT_FOUND,
            "paste not found".to_string(),
        ))
    } else {
        Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain")],
            format!("paste({}) successfully locked", slug),
        ))
    }
}

#[axum::debug_handler]
async fn delete_paste(
    Path(slug): Path<String>,
    Query(map): Query<HashMap<String, String>>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<impl IntoResponse, AppError> {
    let secret = map
        .get("secret")
        .ok_or(AppError(
            StatusCode::BAD_REQUEST,
            "Missing secret".to_string(),
        ))?
        .to_string();

    if Paste::delete(&slug, secret, &pool).await? == 0 {
        Err(AppError(
            StatusCode::NOT_FOUND,
            "paste not found".to_string(),
        ))
    } else {
        Ok((
            StatusCode::OK,
            [(header::CONTENT_TYPE, "text/plain")],
            format!(
                "Post successfully deleted http://{}:8080/{}\n",
                dotenvy::var("HOST").unwrap(),
                slug
            ),
        ))
    }
}
