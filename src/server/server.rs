use axum::{routing::{get}, Router};
use tower_http::{
  services::{ServeDir, ServeFile}
};

pub(crate) fn router() -> Router {
  let router = Router::new().route_service("/", ServeFile::new("playground/index.html")).nest_service("/public", ServeDir::new("playground"));
  router
}

async fn root() -> &'static str {
  "Hello, world!"
}
