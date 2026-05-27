use dioxus::prelude::*;
use dioxus_icons::lucide::{CircleUserRound, ListCollapse};
use dioxus_motion::prelude::*;

use crate::{FAVICON, SVG_ICON_SIZE, nav::*};

#[derive(Routable, Clone, Debug, PartialEq, MotionTransitions)]
#[rustfmt::skip]
pub enum Route {
    #[layout(NavBar)]
        #[route("/")]
        #[transition(Fade)]
        Home {},

        #[route("/about")]
        #[transition(Fade)]
        About {},
        
        #[route("/news/:id")]
        #[transition(Fade)]
        News { id: i32 },

        #[route("/product")]
        #[transition(Fade)]
        Creation {},

        #[route("/dev")]
        #[transition(Fade)]
        Dev {},
        
        // #[route("/slide-left")]
        // #[transition(ZoomIn)]
        
        // #[route("/slide-right")]
        // #[route("/slide-up")]
        // #[route("/slide-down")]
        // #[route("/fade")]

        #[route("/:..route")]
        PageNotFound { route: Vec<String> },

    // #[end_layout]

        // #[route("/")]
        // #[transition(Fade)]
        // BottomContent {},
        
}

/// Shared navbar component.
#[component]
fn NavBar() -> Element {
    let mut be_bar = use_signal(|| 500f64);

    rsx! {
        div {
            id: "navbar",
            class: "horizontal-scroll-wrapper",
            onresize: move |event| {
                let Ok(b) = event.data.get_content_box_size() else {
                    return;
                };
                be_bar.set(b.width);
            },

            img { src: FAVICON, max_height: "90%", margin: "0 3rem 0 3rem" }
            div { flex_grow: 1, max_height: "100%" }

            // if be_bar() < 880.0 {
            //     ListCollapse { style: "margin-right: 1.2rem", size: SVG_ICON_SIZE * 1.4 }
            // } else {
            //     Link { to: Route::Home {}, "Home" }
            //     Link { to: Route::About {}, "About" }
            //     Link { to: Route::News { id: 1 }, "News" }
            //     Link { to: Route::Creation {}, "Creation" }
            //     if cfg!(debug_assertions) {
            //         Link { to: Route::Dev {}, "Dev" }
            //     }
            // }
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::About {}, "About" }
            Link { to: Route::News { id: 1 }, "News" }
            Link { to: Route::Creation {}, "Creation" }
            if cfg!(debug_assertions) {
                Link { to: Route::Dev {}, "Dev" }
            }
            CircleUserRound { style: "margin-right: 1rem", size: "2rem" }

        }

        AnimatedOutlet::<Route> {}
    }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

/// There must be better way to load css style, haven't found
///
/// This is placeholder to preload the style for button relate
#[component]
pub fn First_Loader() -> Element {
    rsx! {
        div { id: "hidden", Dev {} }
    }
}
