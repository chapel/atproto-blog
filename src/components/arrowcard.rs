use leptos::prelude::*;

pub struct Entry {
    collection: String,
    slug: String,
    data: Post,
}

impl Entry {
    pub fn new<S: Into<String>>(collection: S, slug: S, title: S, description: S) -> Entry {
        Entry {
            collection: collection.into(),
            slug: slug.into(),
            data: Post {
                title: title.into(),
                description: description.into(),
            },
        }
    }
}

pub struct Post {
    title: String,
    description: String,
}

#[component]
pub fn ArrowCard(entry: Entry) -> impl IntoView {
    let href = format!("/{}/{}", entry.collection, entry.slug);

    view! {
      <a
        href={href}
        class="flex relative flex-nowrap py-3 px-4 pr-10 rounded-lg border transition-colors duration-300 ease-in-out hover:text-black group border-black/15 dark:border-white/20 dark:hover:bg-white/5 dark:hover:text-white hover:bg-black/5"
      >
        <div class="flex flex-col flex-1 truncate">
          <div class="font-semibold">{entry.data.title}</div>
          <div class="text-sm">{entry.data.description}</div>
        </div>
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 24 24"
          class="absolute right-2 top-1/2 -translate-y-1/2 stroke-current stroke-2 size-5 fill-none"
        >
          <line
            x1="5"
            y1="12"
            x2="19"
            y2="12"
            class="transition-transform duration-300 ease-in-out scale-x-0 translate-x-3 group-hover:scale-x-100 group-hover:translate-x-0"
          />
          <polyline
            points="12 5 19 12 12 19"
            class="transition-transform duration-300 ease-in-out -translate-x-1 group-hover:translate-x-0"
          />
        </svg>
      </a>
    }
}
