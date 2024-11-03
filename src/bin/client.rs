use atproto_blog::app::*;
use leptos::prelude::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(App);
}

pub fn main() {}
