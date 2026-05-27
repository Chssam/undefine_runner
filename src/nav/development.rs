use dioxus::prelude::*;
use dioxus_icons::lucide::*;
use dioxus_motion::prelude::*;
use easer::functions::Easing as _;

use crate::{
    MyState,
    components::{AnimatedMenuItem, BouncingText, Button, ButtonSize, ButtonVariant},
};

#[component]
pub fn Dev() -> Element {
    rsx! {
        Buttoning {}
        Iconing {}
        AnimatedButton {}
        AAAA {}

        BouncingText { text: "Dioxus Motion" }

        for a in 0..50 {
            // Counter {}
            p { "Yeah {a}" }
        }

        AnimatedMenuItem { label: "Yeser" }
    }
}

#[component]
fn Buttoning() -> Element {
    rsx! {
        div { display: "flex", flex_direction: "column", gap: "0.5rem",
            Button { "Primary" }

            Button { variant: ButtonVariant::Secondary, "Secondary" }

            Button { variant: ButtonVariant::Destructive, "Destructive" }

            Button { variant: ButtonVariant::Outline, "Outline" }

            Button { variant: ButtonVariant::Ghost, "Ghost" }

            Button { variant: ButtonVariant::Link, "Link" }
        }
    }
}

#[component]
fn Iconing() -> Element {
    rsx! {
        div {
            display: "flex",
            flex_direction: "row",
            flex_wrap: "wrap",
            align_items: "flex-start",
            gap: "0.75rem",

            Button { variant: ButtonVariant::Outline, size: ButtonSize::Icon,
                ArrowUpRight { size: "16px" }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Icon,
                border_radius: "50%",
                ArrowUpRight { size: "16px" }
            }

            Button {
                variant: ButtonVariant::Outline,
                size: ButtonSize::Sm,
                onmousedown: move |_event| { info!("YEE") },
                GitMerge { size: "16px" }
                "Merge"
            }
        }
    }
}

#[component]
fn Counter() -> Element {
    let mut my_state = use_context::<Signal<MyState>>();
    let read_value = my_state().value;

    rsx! {
        h1 { "Count: {read_value}" }

        div { font_family: "pixFont", "Hello" }
        div { id: "button_icon",
            a { onclick: move |_| my_state.write().value -= 1,
                Minus { size: 32 }
            }
            a { onclick: move |_| my_state.write().value += 1,
                Plus { size: 32 }
            }
            a { onclick: move |_| my_state.set(MyState::default()),
                RefreshCw { size: 32 }
            }
        }

    }
}

#[component]
fn AnimatedButton() -> Element {
    let mut scale = use_motion(1.0f32);

    let hover = move |_| {
        scale.animate_to(
            1.2, // Target value
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let unhover = move |_| {
        scale.animate_to(
            1.0, // Return to original size
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    rsx! {
        button {
            class: "px-4 py-2 bg-blue-500 text-white rounded-sm",
            style: "transform: scale({scale.get_value()})",
            onmouseenter: hover,
            onmouseleave: unhover,
            "Hover me!"
        }
    }
}

#[component]
fn AAAA() -> Element {
    let mut infinite_value = use_motion(0.0f32);

    use_effect(move || {
        infinite_value.animate_to(
            100.0,
            AnimationConfig::new(AnimationMode::Tween(Tween {
                duration: Duration::from_millis(1000),
                easing: easer::functions::Linear::ease_in_out,
            }))
            .with_loop(LoopMode::Alternate),
        );
    });

    rsx! {
        // Infinite loop preview
        div { class: "space-y-2",
            p { class: "text-sm text-text-secondary", "Infinite Loop:" }
            div { class: "relative h-8 bg-dark-200/30 rounded-lg overflow-hidden",
                div {
                    class: "absolute h-8 bg-primary/50 rounded-lg",
                    style: "width: {infinite_value.get_value()}%",
                    height: "50px",
                    background: "pink",
                }
            }
        }
    }
}
