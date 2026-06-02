use dioxus::prelude::*;

#[component]
pub fn About() -> Element {
    let a = [
        ("https://github.com/Chssam", "Created by Chssam"),
        (
            "https://rust-lang.org/",
            "Programmed in modern language, Rust",
        ),
        ("https://dioxuslabs.com/", "Framework by Dioxus"),
        ("https://bevy.org/", "Sponsor to Bevy Game Engine"),
    ];

    rsx! {
        div {
            class: "center",
            flex_direction: "Column",
            margin_bottom: "1rem",
            div {

                h1 { "About Undefine Runner" }

                p { "Has no true meaningful aspect behind this site." }
                p { "This suppose to be just random design, setup, uhhh, that's all I think." }
                p { "Designed to ignore Readability." }
                p { "Created and hosted for the assignment thingy." }
                p { "Dioxus the Framework used." }

                p { "Was planned to use Bevy ECS here but running out of time to implement it," }
                p { "Everything would be easy when I did." }

                h5 { "Who is Chssam?" }
                p { "Started programming in 2020, Javascript is the first programming language learnt." }
                p { "it started from seeing excitement people creating in Discord, the 'Discord Bot'," }
                p { "where programming motivation pushed." }

                p { "Become Rusty doing late 2022, discovered memory safety and efficiency," }
                p { "chosen the language after many repetition comparison which work best," }
                p { "playing existed Rust feature ever exist in Rust economy." }

                p { "Doing fundamental programming is boring, has no goals," }
                p { "decided to think, what if make game would be better," }
                p { "then there's 'Bevy' game engine," }
                p { "all the fundamental of programming and skill are learnt from playing around Bevy." }

            }
        }

        div { class: "center",
            div { id: "links",
                for (linked, content) in a {
                    a { href: "{linked}", target: "_blank", "{content}" }
                }
            }
        }

        div { class: "center",
            div { id: "gif",
                img {
                    class: "active",
                    max_width: "100%",
                    src: "https://tenor.com/view/guaton-computadora-enojado-computer-rage-gif-14480338.gif",
                }
            }
        }

        p { class: "center", "Spend few days to find solution that works to host" }
    }
}
