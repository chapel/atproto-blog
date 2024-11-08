use leptos::prelude::*;

use crate::components::{container::Container, link::Link};

#[component]
pub fn Header() -> impl IntoView {
    view! {
      <header>
        <Container>
          <div class="flex flex-wrap gap-y-2 justify-between">
            <Link href={"/".into()}>
              <div class="font-semibold">"LEPAHC"</div>
            </Link>
            <nav class="flex gap-1">
              <Link href={"/blog".into()}>"blog"</Link>
              <span>"/"</span>
              <Link href={"/work".into()}>"work"</Link>
              <span>"/"</span>
              <Link href={"/projects".into()}>"projects"</Link>
            </nav>
          </div>
        </Container>
      </header>
    }
}
