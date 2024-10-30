use atproto_blog::app::HelloWorld;
use leptos::prelude::mount_to_body;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn hydrate() {
    mount_to_body(HelloWorld);
}

pub fn main() {}
