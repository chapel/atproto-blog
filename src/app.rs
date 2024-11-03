use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{FlatRoutes, Route, Router},
    path, SsrMode,
};

pub fn shell() -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <MetaTags/>
            </head>
            <body>
              <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let fallback = || view! { "Not Found!" }.into_view();

    view! {
      <Stylesheet id="leptos" href="/pkg/styles.css"/>
      <Title text="My Blog"/>
      <Router>
        <nav>
          <a href="/">"Home"</a>
          <a href="/post/1">"Post 1"</a>
        </nav>
        <main>
          <FlatRoutes fallback>
            <Route path=path!("/") view=Home/>
            <Route
              path=path!("/post/:id")
              view=Post
              ssr=SsrMode::Async
            />
          </FlatRoutes>
        </main>
      </Router>
    }
}

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <h1>{"Welcome to My Blog"}</h1>
    }
}

#[component]
pub fn Post() -> impl IntoView {
    view! {
        <h1>{"Post"}</h1>
    }
}
