use leptos::prelude::*;
use leptos_meta::{Meta, Title};

#[component]
pub fn FullTitle<T: Into<String>>(title: T) -> impl IntoView {
    let title_clone = title.into();

    view! {
        <Title text=title_clone.clone() />
        <Meta name="title" content=title_clone />
    }
}
