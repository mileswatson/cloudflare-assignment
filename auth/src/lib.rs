use authenticator::Authenticator;
use worker::*;

mod authenticator;
mod utils;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    let auth = Authenticator::new(&env)?;

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::with_data(auth);

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .get_async("/auth/:username", |req, ctx| async move {
            let username = ctx
                .param("username")
                .ok_or_else(|| Error::RustError("Username not found!".to_string()))?;
            let token = ctx.data().generate_token(username.clone()).await?;
            let cookie = format!("lensflare-auth={}; HttpOnly", token);
            let mut headers = Headers::new();
            headers.set("Set-Cookie", &cookie)?;

            // Fix annoying CORS errors
            headers.set(
                "Access-Control-Allow-Origin",
                &req.headers().get("Origin")?.ok_or_else(|| {
                    worker::Error::RustError("Could not extract host string!".to_string())
                })?,
            )?;
            // ACAO varies depending on the origin
            headers.set("Vary", "Origin")?;
            // Required for CORS preflight request
            headers.set(
                "Access-Control-Allow-Headers",
                "X-Requested-With, Content-Type, Accept, Origin, Authorization",
            )?;

            Response::empty().map(|res| res.with_headers(headers))
        })
        .run(req, env)
        .await
}
