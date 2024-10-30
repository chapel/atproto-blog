use leptos::prelude::*;
use leptos_meta::provide_meta_context;

#[component]
pub fn hello_world() -> impl IntoView {
    provide_meta_context();

    view! {
      <h1>"Hello, World! Everyone!"</h1>
    }
}
