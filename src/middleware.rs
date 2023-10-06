use reqwest::header::{HeaderMap};
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use task_local_extensions::Extensions;

fn convert(headers: &HeaderMap) -> serde_json::Value {
    format!("{:?}", headers).into()
}

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        println!("Request to {} {}", req.method(), req.url());
        println!("Headers: {}", convert(req.headers()));
        let resp = next.run(req, extensions).await?;
        println!("Got response {}", resp.status());
        Ok(resp)
    }
}
