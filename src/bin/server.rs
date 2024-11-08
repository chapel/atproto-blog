use axum::routing::get;
use axum::Router;
use leptos::prelude::{expect_context, provide_context, LeptosOptions};
use leptos_axum::{generate_route_list, LeptosRoutes};
use tower::Service;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use worker::*;

use atproto_blog::app::*;
use atproto_blog::libs::atproto::{get_posts, Records};
use atproto_blog::libs::state::*;

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
    let app_state = AppState {
        fetch_proxy: FetchProxy::new(move |url, sender| {
            let url = url.map(|u| u.to_string());
            spawn_local(async {
                web_sys::console::log_1(&"fetch_proxy start".into());
                let resp = Fetch::Url(url.unwrap().parse().unwrap())
                    .send()
                    .await
                    .expect("Failed to fetch");
                sender
                    .send(std::result::Result::Ok(resp.into()))
                    .expect("Failed to send response");
                web_sys::console::log_1(&"fetch_proxy end".into());
            });
        }),
        leptos_options,
    };

    let resp = app_state.fetch_proxy.send_with_url("https://shiitake.us-east.host.bsky.network/xrpc/com.atproto.repo.listRecords?repo=did:plc:zqduigrgzgbxnhhnwujnpzo2&collection=com.whtwnd.blog.entry").await.unwrap();
    web_sys::console::log_1(&format!("{:?}", resp).into());

    async fn get_posts_handler() -> axum::Json<Records> {
        let posts = get_posts().await.unwrap();
        web_sys::console::log_1(&format!("{:?}", posts).into());
        axum::Json::from(posts)
    }

    let mut router = Router::new()
        .route(
            "/api/get_posts",
            get(|| async { 
    let state = expect_context::<AppState>();
    web_sys::console::log_1(&"expect_context end".into());
    let AppState { fetch_proxy, .. } = state;
                let response = fetch_proxy.send_with_url("https://shiitake.us-east.host.bsky.network/xrpc/com.atproto.repo.listRecords?repo=did:plc:zqduigrgzgbxnhhnwujnpzo2&collection=com.whtwnd.blog.entry").await.unwrap();
                let body = response.body().unwrap().to_string();
                axum::response::Response::new(axum::body::Body::new::<String>(body.into()))
            }),
        )
        .leptos_routes_with_context(
            &app_state,
            generate_route_list(App),
            {
                let app_state = app_state.clone();
                move || provide_context(app_state.clone())
            },
            shell,
        )
        .with_state(app_state.clone());
    Ok(router.call(req).await?)
}

pub fn main() {}
