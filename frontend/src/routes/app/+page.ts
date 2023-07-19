import type { PageLoad } from './$types';

export const load = (async ({ fetch }) => {
    const res = await fetch(`/api/test`);
    const userInfo = await res.json();
    console.log(userInfo);
    return {
        post: {
            name: userInfo.name,
            content: `Content for test goes here`
        }
    };
}) satisfies PageLoad;
