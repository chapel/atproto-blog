use leptos::prelude::*;

#[component]
pub fn Container(children: Children) -> impl IntoView {
    view! { <div class="mx-auto max-w-screen-sm px-5">{children()}</div> }
}
