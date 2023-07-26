import type { PageServerLoad } from './$types';
import { DATABASE_URL } from '$env/static/private';
import Database from 'better-sqlite3';

export const load: PageServerLoad = async () => {
    const db = new Database(DATABASE_URL.replace(new RegExp('sqlite:'), ''));
    return {
        posts: {
            loaded: db.prepare('SELECT * FROM pastes').all(),
        }
    }
}

