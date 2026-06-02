use dioxus::prelude::*;

use crate::nav::Route;

/// News page
#[component]
pub fn News(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "News" }
            p { "Hi Nope" }

            // Navigation links
            Link { to: Route::News { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::News { id: id + 1 }, "Next" }
        }
    }
}
