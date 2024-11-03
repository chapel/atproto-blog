use axum::Router;
use leptos::prelude::LeptosOptions;
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::Service;
use worker::*;

use atproto_blog::app::*;

#[event(fetch)]
pub async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<axum::http::Response<axum::body::Body>> {
    let leptos_options = LeptosOptions::builder()
        .output_name("index")
        .site_pkg_dir("pkg")
        .build();

    let mut router = Router::new()
        .leptos_routes(&leptos_options, generate_route_list(App), shell)
        .with_state(leptos_options);
    Ok(router.call(req).await?)
}

pub fn main() {}
