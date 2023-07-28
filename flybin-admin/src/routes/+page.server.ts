import type { PageServerLoad } from './$types';
import { error, redirect } from '@sveltejs/kit';
import { getAllPastes, getSession, logout } from '$lib/db';

export const load = (async ({ cookies }) => {
    const session_token = cookies.get('session');
    if (await getSession(session_token)) {
        return {
            pastes: {
                loaded: getAllPastes(),
            }
        }
    } else {
        throw error(401,{ 
            message:  'Not logged in' 
        });
    }
}) satisfies PageServerLoad;

export const actions = {
    default: async ({ cookies }) => {
        const session_token = cookies.get('session');
        if (await getSession(session_token)) {
            await logout(session_token);
        }
        cookies.delete('session');
        throw redirect(303, '/login');
    }
} satisfies Actions;
