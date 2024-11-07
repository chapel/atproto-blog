use leptos::prelude::*;

use crate::components::container::Container;

#[component]
pub fn Header() -> impl IntoView {
    view! {
      <header>
        <Container>
          <div class="flex flex-wrap gap-y-2 justify-between">
            <a href="/">
              <div class="font-semibold">"LEPAHC"</div>
            </a>
            <nav class="flex gap-1">
              <a href="/blog">"blog"</a>
              <span>"/"</span>
              <a href="/work">"work"</a>
              <span>"/"</span>
              <a href="/projects">"projects"</a>
            </nav>
          </div>
        </Container>
      </header>
    }
}
