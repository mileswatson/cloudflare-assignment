mod post;
mod utils;

use futures::future;
use rand::Rng;
use worker::*;

use post::Post;

fn log_request(req: &Request) {
    console_log!(
        "{} - [{}], located at: {:?}, within: {}",
        Date::now().to_string(),
        req.path(),
        req.cf().coordinates().unwrap_or_default(),
        req.cf().region().unwrap_or_else(|| "unknown region".into())
    );
}

fn set_headers(req: Request, res: Response, content_type: &str) -> Result<Response> {
    let mut headers = Headers::new();
    headers.set("Content-Type", content_type)?;
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
    Ok(res.with_headers(headers))
}

#[event(fetch)]
pub async fn main(req: Request, env: Env) -> Result<Response> {
    log_request(&req);

    // Get kv store from configuration
    let posts = env.kv("ASSIGNMENT_KV")?;

    // Optionally, get more helpful error messages written to the console in the case of a panic.
    utils::set_panic_hook();

    // Optionally, use the Router to handle matching endpoints, use ":name" placeholders, or "*name"
    // catch-alls to match on specific patterns. Alternatively, use `Router::with_data(D)` to
    // provide arbitrary data that will be accessible in each route via the `ctx.data()` method.
    let router = Router::with_data(posts);

    // Add as many routes as your Worker needs! Each route will get a `Request` for handling HTTP
    // functionality and a `RouteContext` which you can use to  and get route parameters and
    // Environment bindings like KV Stores, Durable Objects, Secrets, and Variables.
    router
        .get("/", |_, _| Response::ok("Hello from Workers!"))
        .options("/posts", |req, _| {
            Response::empty().and_then(|res| set_headers(req, res, "text/plain"))
        })
        .get_async("/posts", |req, ctx| async move {
            let posts = ctx.data();
            let keys = posts.list().execute().await.map(|x| x.keys);
            let values: Vec<Post> = future::join_all(keys?.iter().map(|k| posts.get(&k.name)))
                .await
                .into_iter()
                // filter out any errors or missing values
                .filter_map(|result| result.unwrap_or(None))
                // attempt to parse JSON, filtering out any failing values
                .filter_map(|x| serde_json::from_str(&x.as_string()).ok())
                .collect();
            Response::ok(serde_json::to_string(&values).unwrap())
                .and_then(|res| set_headers(req, res, "application/json"))
        })
        .post_async("/posts", |mut req, ctx| async move {
            let form: Post = match req.json().await {
                Err(_) => return Response::error("Invalid format", 400),
                Ok(x) => x,
            };
            if let Err(e) = form.validate() {
                return Response::error(e, 400).and_then(|res| set_headers(req, res, "text/plain"));
            }
            let posts = ctx.data();
            let rand_id = rand::thread_rng().gen::<u128>().to_string();
            posts
                .put(&rand_id, serde_json::to_string(&form)?)?
                .execute()
                .await?;
            Response::ok("success").and_then(|res| set_headers(req, res, "text/plain"))
        })
        .run(req, env)
        .await
}
