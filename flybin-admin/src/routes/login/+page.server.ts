import type { Actions } from './$types';
import { login } from '$lib/db';
import { fail, redirect } from '@sveltejs/kit';

export const actions = {
    default: async ({ request, cookies }) => {
        const formData = await request.formData();
        const username = formData.get('username');
        const password = formData.get('password');

        if (username && password) {
            const session = await login(username.toString(), password.toString());
            if (session) {
                cookies.set('session', session, { path: "/", httpOnly: true, sameSite: 'strict' });
            } else {
                return fail(401, { message: 'Invalid credentials' });
            }
        } else {
            return fail(401, { message: 'Invalid credentials' });
        }
        throw redirect(303, '/');
    }
} satisfies Actions;
