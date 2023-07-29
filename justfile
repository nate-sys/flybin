host := "localhost"
http-port := "8080"
nc-port := "9999"

set dotenv-load
db_url := trim_start_match(env_var("DATABASE_URL"), "sqlite:")


# Rerun migrations 
reset:
    sqlx database reset

# Start the main server
serve: 
    cargo r --bin flybin-server

# create a paste
paste PASTE:
    echo {{PASTE}} | nc {{host}} {{nc-port}}

# get a paste : just get MySlug [password=something]
get SLUG *PASSARGS:
    curl -G \
        -d "{{PASSARGS}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}

# get a syntax highlighted paste : just getlang rust MySlug [password=something]
getlang LANG SLUG *PASSARGS: 
    curl -G \
        -d "{{PASSARGS}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}/{{LANG}}

# delete a paste : just delete MySlug MySecretToken
delete SLUG SECRET :
    curl -X DELETE -G \
        -d "secret={{SECRET}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}


# lock a paste : just lock MySlug MySecretToken MyPassword
lock SLUG SECRET PASSWORD:
    curl -X POST -G \
        -d "password={{PASSWORD}}" \
        -d "secret={{SECRET}}" \
        http://{{host}}:{{http-port}}/{{SLUG}}

# seed the pastebin with contents of files in the current dir
seed:
    fd -t f | while read -r file; do cat "$file" | nc {{host}} {{nc-port}} ; done

# create an admin : just admin username password
admin USERNAME PASSWORD:
    #!/usr/bin/env bash
    pass_hash=$(botan gen_argon2 --t=3 --p=4 {{PASSWORD}})
    sqlite3 {{db_url}} \
        "insert into admins (username, password) values ('{{USERNAME}}', '${pass_hash}');"
