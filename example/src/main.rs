use axum::{http::StatusCode, response::IntoResponse, routing::get, Form, Router};
use nidrs::{Exception, StateCtx};
use nidrs_extern::colored::Colorize;


mod app;
mod conf;
mod user;
mod log;
mod shared;

pub use nidrs::AppResult;
pub use nidrs::AppError;

#[nidrs::main]
fn main() {
    let mut app = nidrs::NidrsFactory::create(app::AppModule);

    app.router = app.router.merge(Router::<StateCtx>::new().route("/api", get(|| async { "Hello, World!" })));
    
    let app = app.listen::<AppError>(3000);
    let _ = tokio::runtime::Runtime::new().unwrap().block_on(app);
}


