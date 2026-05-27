use dioxus::prelude::*;
use dioxus_icons::lucide::ArrowUpRight;

use crate::{
    LOGO,
    components::{Button, ButtonSize, ButtonVariant},
    nav::Route,
};

/// Home page
#[component]
pub fn Home() -> Element {
    rsx! {
        Hero {}
    }
}

#[component]
fn Hero() -> Element {
    rsx! {

        div { class: "center", style: "flex-wrap: wrap;",
            div {
                class: "center",
                style: "flex-direction: column; align-items: flex-start;",
                color: "#43d1f2",
                h1 { style: "font-size: 2.5rem", "Undefine Runner" }

                p {
                    style: "font-size: 1.4rem",
                    margin_top: "0.5rem",
                    margin_bottom: "2.6rem",
                    color: "#f60ff8",
                    "Get handcrafted creation by the creator"
                }

                div { id: "link_to",
                    Link { to: Route::Creation {}, "Creation" }
                    ArrowUpRight { size: "32px" }
                }

            }
            img {
                src: LOGO,
                id: "header",
                width: "400px",
                height: "400px",
            }
        }

        // ToastProvider {
        //     button {
        //         onclick: |event: MouseEvent| {
        //             // Consume the toast context to send a toast.
        //             let toast_api = consume_toast();
        //             toast_api
        //                 .error(
        //                     "Critical Error".to_string(),
        //                     ToastOptions::new()
        //                         .description("Some info you need")
        //                         .duration(Duration::from_secs(60))
        //                         .permanent(false),
        //                 );
        //         },
        //         "Show Toast"
        //     }
        // }
    }
}
