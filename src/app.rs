use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path, SsrMode,
};

use crate::components::{header::Header, meta::FullTitle};
use crate::pages::home::Home;

pub fn shell() -> impl IntoView {
    view! {
      <!DOCTYPE html>
      <html lang="en">
        <head>
          <meta charset="utf-8" />
          <meta name="viewport" content="width=device-width, initial-scale=1" />
          <MetaTags />
        </head>
        <body>
          <App />
        </body>
      </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let fallback = || view! { "Not Found!" }.into_view();

    view! {
      <Stylesheet id="leptos" href="/pkg/styles.css" />
      <FullTitle title="My Blog" />
      <Router>
        <Header />
        <main>
          <FlatRoutes fallback>
            <Route path={path!("/")} view={Home} />
            <Route path={path!("/posts/:id")} view={Post} ssr={SsrMode::Async} />
          </FlatRoutes>
        </main>
      </Router>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    view! { <h1>{"Post"}</h1> }
}
