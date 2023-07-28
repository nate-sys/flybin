import { DATABASE_URL } from '$env/static/private';
import Database from 'better-sqlite3';
import argon2 from 'argon2';
import {v4 as uuidv4} from 'uuid';

const db = new Database(DATABASE_URL.replace(new RegExp('sqlite:'), ''));

export async function getAllPastes() {
    return db.prepare('SELECT * FROM pastes').all();
}

export async function getSession(session_token: string)  {
    if (session_token) {
        try {
            return await db.prepare('SELECT * FROM sessions WHERE token = ?').get(session_token);
        } catch (e) {
            return null;
        } 
    }
    return null;
}

export async function login(username: string, password: string)  {
    try {
        const admin = await db.prepare('SELECT id, password FROM admins WHERE username = ?').get(username);
        if (await argon2.verify(admin.password, password)) {
            const session_token = uuidv4();   
            let resp = await db.prepare('INSERT INTO sessions (id, token) VALUES (?, ?)').run(admin.id, session_token);
            return session_token;
        }
    } catch (e) {
        return null;
    } 
}

export async function logout(session_token)  {
    try {
        await db.prepare('DELETE FROM sessions WHERE token = ?').get(token);
    } catch (e) {
        return null;
    } 
}
