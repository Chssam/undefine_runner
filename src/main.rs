use dioxus::prelude::*;
use dioxus_motion::prelude::*;

mod components;
mod nav;
use easer::functions::Easing as _;
use nav::*;

pub const FAVICON: Asset = asset!("/assets/Undefine Runner favicon.ico");
pub const LOGO: Asset = asset!("/assets/Undefine Runner Logo.svg");
pub const MAIN_CSS: Asset = asset!("/assets/main.css");
pub const THEME_CSS: Asset = asset!("/assets/components-theme.css");
pub const PIX_FONT: Asset = asset!("/assets/fonts/DepartureMono-1.500/DepartureMono-Regular.woff2");

pub const BUTTON_CSS: Asset = asset!("/src/components/button/style.css");
pub const TOAST_CSS: Asset = asset!("/src/components/toast/style.css");

pub const PONK: Asset = asset!("/assets/Ponk.png");

pub const PEEK: Asset = asset!("/assets/peek/Peek.png");
pub const PEEK_UP: Asset = asset!("/assets/peek/Peek Up.png");
pub const PEEK_LEFT: Asset = asset!("/assets/peek/Peek Left.png");
pub const PEEK_RIGHT: Asset = asset!("/assets/peek/Peek Right.png");
pub const PEEK_TOP_LEFT: Asset = asset!("/assets/peek/Peek Top Left.png");
pub const PEEK_TOP_RIGHT: Asset = asset!("/assets/peek/Peek Top Right.png");

pub const SVG_ICON_SIZE: f32 = 32.0;

#[derive(Clone, Default)]
pub struct MyState {
    pub value: i32,
}

#[derive(Clone, Default)]
pub struct User {
    pub mode: UserMode,
}

#[derive(Clone, Default)]
pub enum UserMode {
    Admin,
    ProductCreator,
    User,
    #[default]
    Guest,
}

pub static USER: GlobalSignal<User> = Signal::global(|| User::default());

fn main() {
    LaunchBuilder::new().launch(App);
}

#[component]
fn App() -> Element {
    let tween = use_store(|| Tween {
        duration: std::time::Duration::from_millis(500),
        easing: easer::functions::Cubic::ease_in_out,
    });
    use_context_provider(move || tween);
    use_context_provider(|| Signal::new(MyState::default()));
    use_context_provider(|| Signal::new(FilterState::default()));
    use_context_provider(|| Signal::new(ActiveItem::default()));

    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: THEME_CSS }

        document::Link { rel: "stylesheet", href: BUTTON_CSS }
        document::Link { rel: "stylesheet", href: TOAST_CSS }

        document::Link { rel: "icon", href: FAVICON }

        style {
            "@font-face {{ font-family: 'departureMono'; src: url('{PIX_FONT}') format('woff2'); font-weight: normal; font-style: normal; font-display: swap; }}"
        }

        First_Loader {}
        Playful {}

        Router::<Route> {}
    }
}

pub struct CursorPos {
    pub x: f64,
    pub y: f64,
}

#[component]
fn Playful() -> Element {
    let mut cur_pos = use_signal(|| (0f64, 0f64));
    let mut scale = use_motion(6f32);

    let hover = move |_| {
        scale.animate_to(
            9.5,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    let unhover = move |_| {
        scale.animate_to(
            6.0,
            AnimationConfig::new(AnimationMode::Spring(Spring::default())),
        );
    };

    // use_effect(move || {
    // });

    rsx! {
        div {
            left: "0px",
            top: "0px",
            position: "fixed",
            width: "100%",
            height: "100%",
            z_index: 1000,
            pointer_events: "none",
            onmousemove: |event| {
                info!(? event);
                info!("{:?}", event.client_coordinates());
            },
        }

        div {
            img {
                id: "playful",
                bottom: "0px",
                left: "40px",
                src: PEEK,
                transform: "scale(6)",
                transform_origin: "left bottom",
            }

            img {
                id: "playful",
                bottom: "50px",
                right: "70px",
                src: PONK,
                transform: {
                    let v = scale.get_value();
                    format!("scale(-{v}, {v})")
                },
                onmouseenter: hover,
                onmouseleave: unhover,
            }
        }

    }
}
