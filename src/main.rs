mod document_parsing;
use axum::{http::StatusCode, response::Html, routing::get, routing::get_service, Router};
use document_parsing::md_to_html;
use log::{error, info};
use std::fs;

use lazy_static::lazy_static;
use std::io;
use tera::{Context, Tera};
use tower_http::services::ServeFile;

// process tera templates
lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![]);
        tera
    };
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/", get(root))
        .route(
            "/style.css",
            get_service(ServeFile::new("templates/style.css")).handle_error(
                |error: io::Error| async move {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Interal error: {}", error),
                    )
                },
            ),
        )
        .fallback(fallback);

    axum::Server::bind(&"0.0.0.0:80".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
async fn root() -> Html<String> {
    info!("/main request");
    let mut context = Context::new();
    let path = "test.md".to_string();
    let contents = fs::read_to_string(path).unwrap();
    let html = md_to_html(contents).expect("index parsing error");
    context.insert("content", &html);

    let main = TEMPLATES.render("main.html", &context).expect("fail");
    Html(main)
}

async fn fallback() -> (StatusCode, &'static str) {
    error!("failed to find thing?");
    (StatusCode::NOT_FOUND, "Not Found")
}
