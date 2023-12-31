use log::info;
use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next};
use task_local_extensions::Extensions;

pub struct LoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> reqwest_middleware::Result<Response> {
        info!("Request to {} {}", req.method(), req.url());

        let resp = next.run(req, extensions).await?;

        info!("Received response with status: {}", resp.status());
        Ok(resp)
    }
}
