const API = {
    async _req(method, path, body) {
        const token = localStorage.getItem('jwt');
        const headers = { 'Content-Type': 'application/json' };
        if (token) headers['Authorization'] = `Bearer ${token}`;

        const res = await fetch(path, {
            method,
            headers,
            body: body !== undefined ? JSON.stringify(body) : undefined,
        });

        if (res.status === 204) return null;

        const data = await res.json().catch(() => ({ error: '응답 파싱 실패' }));
        if (!res.ok) throw new Error(data.error || `오류 (${res.status})`);
        return data;
    },

    auth: {
        register: (username, password) =>
            API._req('POST', '/auth/register', { username, password }),
        login: (username, password) =>
            API._req('POST', '/auth/login', { username, password }),
    },

    posts: {
        list:   (page = 1, per_page = 10) =>
            API._req('GET', `/posts?page=${page}&per_page=${per_page}`),
        get:    (id) => API._req('GET', `/posts/${id}`),
        create: (title, content) =>
            API._req('POST', '/posts', { title, content }),
        update: (id, title, content) =>
            API._req('PUT', `/posts/${id}`, { title, content }),
        delete: (id) => API._req('DELETE', `/posts/${id}`),
    },
};

function getMe() {
    return {
        id:       parseInt(localStorage.getItem('userId') || '0'),
        username: localStorage.getItem('username') || '',
        jwt:      localStorage.getItem('jwt'),
    };
}

function requireAuth() {
    if (!localStorage.getItem('jwt')) {
        location.href = '/login.html';
        return false;
    }
    return true;
}

function escapeHtml(str) {
    return String(str)
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/"/g, '&quot;');
}

function formatDate(str) {
    if (!str) return '';
    return new Date(str).toLocaleString('ko-KR', {
        year: 'numeric', month: '2-digit', day: '2-digit',
        hour: '2-digit', minute: '2-digit',
    });
}
