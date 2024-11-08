use axum::extract::FromRef;
use futures::channel::oneshot::{channel, Sender};
use leptos::prelude::LeptosOptions;
use worker::Error;

#[derive(Clone)]
pub struct FetchProxy {
    proxy: fn(
        url: Option<&str>,
        //req: Option<Request>,
        sender: Sender<Result<web_sys::Response, Error>>,
    ),
}

impl FetchProxy {
    pub fn new(
        proxy: fn(
            url: Option<&str>,
            //req: Option<Request>,
            sender: Sender<Result<web_sys::Response, Error>>,
        ),
    ) -> Self {
        Self { proxy }
    }

    #[worker::send]
    pub async fn send_with_url(&self, url: &str) -> Result<web_sys::Response, Error> {
        web_sys::console::log_1(&"send_with_url start".into());
        let (tx, rx) = channel();

        (self.proxy)(Some(url), tx);
        rx.await
            .map_err(|_| Error::RustError("Callback error".into()))?
    }

    /*
    pub async fn send_with_req(&self, req: Request) -> Result<web_sys::Response, Error> {
        web_sys::console::log_1(&"send_with_req start".into());
        let (tx, rx) = channel();
        (self.proxy)(None, Some(req), tx);
        rx.await
            .map_err(|_| Error::RustError("Callback error".into()))?
    }
    */
}

#[derive(FromRef, Clone)]
pub struct AppState {
    pub fetch_proxy: FetchProxy,
    pub leptos_options: LeptosOptions,
}
