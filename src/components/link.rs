use leptos::prelude::*;

#[component]
pub fn Link(
    href: String,
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let class = class.unwrap_or_else(|| "inline-block text-current underline transition-colors duration-300 ease-in-out hover:text-black decoration-black/15 underline-offset-2 dark:decoration-white/30 hover:decoration-black/25 hover:dark:decoration-white/50 hover:dark:text-white".into());

    view! {
      <a href={href} class={class}>
        {children()}
      </a>
    }
}
