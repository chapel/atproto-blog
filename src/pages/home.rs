use leptos::prelude::*;

use crate::components::{
    arrowcard::{ArrowCard, Entry},
    container::Container,
    link::Link,
};

#[component]
pub fn Home() -> impl IntoView {
    let post: Entry = Entry::new(
        "posts",
        "hello-world",
        "Hello, World!",
        "This is a test post",
    );

    view! {
      <Container>
        <h4 class="font-semibold text-black dark:text-white">
          "Hi, I'm Nano "<span class="text-xl">"👋🏻"</span>
        </h4>
        <div class="space-y-16">
          <section>
            <article class="space-y-4">
              <p>
                "I am a minimal, seo friendly, accessible portfolio and blog for developers."
                "I am an even more lightweight version of my popular theme "
                <Link
                  href={"https://github.com/markhorn-dev/astro-sphere".into()}
                  {..}
                  aria-label="Astro Sphere on Github"
                >
                  "Astro Sphere"
                </Link>.
                " I have a lighthouse scores of 100 across the board for performance, accesibility, best practices and SEO."
              </p>
              <p>
                "I come packed with full type safety, a sitemap, an rss feed, markdown and mdx support (use components in your markdown) through astro integrations. I am styled with tailwind and come preconfigured with light, dark and system/os theme preferences out of the box."
              </p>
              <p>
                "Visit "
                <Link
                  href={"https://github.com/markhorn-dev/astro-nano".into()}
                  {..}
                  aria-label="Astro Nano on Github"
                >
                  "Astro Nano on Github "
                </Link>
                "to fork the repository, read the docs or one-click deploy on Vercel or Netlify."
              </p>
            </article>
          </section>
          <section class="space-y-6">
            <div class="flex flex-wrap gap-y-2 justify-between items-center">
              <h5 class="font-semibold text-black dark:text-white">Latest posts</h5>
              <Link href={"/blog".into()}>"See all posts"</Link>
            </div>
            <ul class="flex flex-col gap-4">
              <li>
                <ArrowCard entry={post} />
              </li>
            </ul>
          </section>
        </div>
      </Container>
    }
}
