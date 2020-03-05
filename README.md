# A proof-of-concept for Cloudflare Workers and Workers KV

Pretty much the title, view it, but build your own. It's not made to be robust or tidy or useable.

The story is documented on [here on Medium](https://medium.com/@psiphi75/rust-and-serverless-with-a-focus-on-cloudflare-workers-342effbc4f85).

## Secrets

You need to add the Worker KV API keys as environment variables to your Worker's runtime. Once
added these will be available to the Worker.

```sh
wrangler secret put KV_ACCOUNT_ID "Your Workers Account Id"
wrangler secret put KV_NAMESPACE_ID  "The Worker KV namespace Id"
wrangler secret put KV_AUTH_EMAIL "The email address for your account"
wrangler secret put KV_AUTH_KEY "The Auth Key for the given email address"
```

## 🚴 Usage

### 🐑 Use `wrangler generate` to Clone this Template

[Learn more about `wrangler generate` here.](https://github.com/cloudflare/wrangler)

```
wrangler generate wasm-worker  https://github.com/cloudflare/rustwasm-worker-template.git
cd wasm-worker
```

### 🛠️ Build with `wasm-pack build`

```
wasm-pack build
```

### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```
