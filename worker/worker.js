addEventListener('fetch', event => {
    event.respondWith(handleRequest(event.request));
});

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
    const { respond2_wrapper } = wasm_bindgen;
    await wasm_bindgen(wasm);
    const t = new Date().getTime();
    console.time('WorkerTimer');
    console.log(WORKER_KV);

    let body;
    if (request.body) {
        body = await request.text();
    } else {
        body = '';
    }

    let headers = {};
    for (let key of request.headers.keys()) {
        headers[key] = request.headers.get(key);
    }

    const response = await respond2_wrapper({
        method: request.method,
        headers: headers,
        url: request.url,
        body: body,
        kv_account_id: KV_ACCOUNT_ID,
        kv_namespace_id: KV_NAMESPACE_ID,
        kv_auth_email: KV_AUTH_EMAIL,
        kv_auth_key: KV_AUTH_KEY
    });
    console.log(response);
    // response.body += ` time ${new Date().getTime() - t}`;
    // console.log(response.body);
    console.timeEnd('WorkerTimer');

    return new Response(response.body, {
        status: response.status,
        headers: response.headers
    });
}
