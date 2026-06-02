use std::collections::HashSet;

use convert_case::ccase;
use dioxus::prelude::*;
use dioxus_primitives::{ContentSide, checkbox::CheckboxState};
use strum::{EnumCount, IntoEnumIterator};

use crate::{FAVICON, components::*};

#[derive(Clone, Debug, Default)]
pub struct FilterState {
    sort_by: SortBy,
    item_tags: Vec<ItemTags>,
    creator_name: String,
    item_name: String,
}

#[derive(Clone, Debug)]
pub struct ActiveItem(pub Vec<Item>);

impl Default for ActiveItem {
    fn default() -> Self {
        Self(vec![
            Item {
                img: "https://cdn.discordapp.com/emojis/1276281009167667363.webp?size=44".to_owned(),
                name: "Ponk".to_owned(),
                creator: "Chssam".to_owned(),
                quantity: 5,
                cost: 5.4,
                tags: vec![ItemTags::Decor],
                ..Default::default()
            },
            Item {
                img: "https://cdn.discordapp.com/emojis/1276281009167667363.webp?size=44".to_owned(),
                name: "Flower Wrist".to_owned(),
                creator: "Nameless".to_owned(),
                quantity: 10,
                cost: 10.0,
                tags: vec![ItemTags::Bracelet, ItemTags::Customizable],
                ..Default::default()
            },
            Item {
                img: "https://cdn.discordapp.com/emojis/1276281009167667363.webp?size=44".to_owned(),
                name: "Something Else".to_owned(),
                creator: "Black Marketer".to_owned(),
                quantity: 120,
                cost: 555.2,
                tags: vec![ItemTags::Decor, ItemTags::Corsage],
                ..Default::default()
            },
            Item {
                img: "https://cdn.discordapp.com/attachments/976796820095266826/1283736024731881472/Scaled_Ponk.png?ex=6a177016&is=6a161e96&hm=d851e18c0bce7062257d115c29f1f31f8dacfd85830948b2d3c7b74ec7a47373&".to_owned(),
                name: "Big Ponk".to_owned(),
                creator: "Nova".to_owned(),
                quantity: 6,
                cost: 15.5,
                tags: vec![ItemTags::Decor, ItemTags::Customizable, ItemTags::Large],
                ..Default::default()
            },
        ])
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Item {
    pub img: String,
    pub name: String,
    pub creator: String,
    pub quantity: u16,
    pub cost: f32,
    pub tags: Vec<ItemTags>,
    pub taking: u16,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::AsRefStr,
)]
pub enum ItemTags {
    Customizable,
    Corsage,
    Bracelet,
    Decor,
    Large,
}

impl ItemTags {
    const fn emoji(&self) -> &'static str {
        match self {
            ItemTags::Customizable => "⚒️",
            ItemTags::Corsage => "🌸",
            ItemTags::Bracelet => "⌚",
            ItemTags::Decor => "💎",
            ItemTags::Large => "🔷",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    Default,
    PartialEq,
    strum::EnumCount,
    strum::EnumIter,
    strum::Display,
    strum::AsRefStr,
)]
pub enum SortBy {
    QuantityLowHigh,
    QuantityHightLow,
    CostLowHigh,
    CostHighLow,
    #[default]
    None,
}

impl SortBy {
    const fn emoji(&self) -> &'static str {
        match self {
            SortBy::QuantityLowHigh => "📈",
            SortBy::QuantityHightLow => "📉",
            SortBy::CostLowHigh => "💸",
            SortBy::CostHighLow => "💸",
            SortBy::None => "❌",
        }
    }
}

/// Creation page
#[component]
pub fn Creation() -> Element {
    let mut state = use_context::<Signal<FilterState>>();
    let mut active_item = use_context::<Signal<ActiveItem>>();
    let mut show_taking_only = use_signal(|| false);

    let output = use_memo(move || {
        let filt_state = state();
        let active_items = active_item();
        let mut new_item: Vec<(Item, usize)> = Vec::new();
        let mut cost = 0f32;

        for (indexed, item) in active_items.0.iter().enumerate() {
            if show_taking_only() && item.taking == 0 {
                continue;
            }

            if !filt_state.item_name.is_empty() {
                if !item.name.contains(&filt_state.item_name) {
                    continue;
                }
            }

            if !filt_state.creator_name.is_empty() {
                if !item.creator.contains(&filt_state.creator_name) {
                    continue;
                }
            }

            if !filt_state
                .item_tags
                .iter()
                .all(|need_tag| item.tags.contains(need_tag))
            {
                continue;
            }

            cost += item.cost * item.taking as f32;

            new_item.push((item.clone(), indexed));
        }

        match filt_state.sort_by {
            SortBy::QuantityLowHigh => new_item.sort_by(|a, b| a.0.quantity.cmp(&b.0.quantity)),
            SortBy::QuantityHightLow => new_item.sort_by(|a, b| b.0.quantity.cmp(&a.0.quantity)),
            SortBy::CostLowHigh => {
                new_item.sort_by(|a, b| a.0.cost.partial_cmp(&b.0.cost).unwrap())
            }
            SortBy::CostHighLow => {
                new_item.sort_by(|a, b| b.0.cost.partial_cmp(&a.0.cost).unwrap())
            }
            SortBy::None => {}
        }

        (new_item, cost)
    });

    rsx! {

        div {
            class: "center",
            align_items: "flex-start",
            flex_direction: "column",
            // width: "stretch",
            overflow: "scroll",

            div { class: "center",
                h1 { text_wrap: "nowrap", "Hello World, This Is Creation Page" }
            }

            div {
                class: "center",
                flex_direction: "row",
                justify_content: "flex-start",
                align_items: "flex-start",
                // width: "stretch",
                max_width: "100vh",

                div { id: "contained", margin_right: "2rem",

                    Sort_By_Tag {}

                    h1 { class: "center", "Filter Zone" }

                    Item_Tag {}

                    Tooltip {
                        TooltipTrigger { style: "margin-top: 1rem", "Creator name" }
                        TooltipContent { side: ContentSide::Top, style: "width: 200px",
                            p { color: "black", "(Case sensitive)" }
                        }
                    }

                    input {
                        color: "black",
                        value: "{state().creator_name}",
                        oninput: move |event| state.write().creator_name = event.value(),
                    }

                    Tooltip {
                        TooltipTrigger { style: "margin-top: 1rem", "Item name" }
                        TooltipContent { side: ContentSide::Top, style: "width: 200px",
                            p { color: "black", "(Case sensitive)" }
                        }
                    }

                    input {
                        color: "black",
                        value: "{state().item_name}",
                        oninput: move |event| state.write().item_name = event.value(),
                    }

                    div {
                        class: "center",
                        justify_content: "flex-start",
                        margin_top: "1rem",
                        a { "Show ordering item" }

                        Checkbox {
                            style: "width: 2rem; height: 2rem; margin-left: 1rem",
                            on_checked_change: move |event| {
                                show_taking_only.set(matches!(event, CheckboxState::Checked));
                            },
                            name: "show_taking_only",
                        }
                    }
                }

                div {
                    class: "center",
                    flex_shrink: 0,
                    flex_grow: 1,
                    flex_direction: "column",
                    min_width: "100%",

                    div { id: "contained", width: "stretch",
                        a { margin_left: "6rem", flex_grow: 1, "Item name" }
                        a { margin_left: "8.5em", width: "5rem", "Creator" }
                        a {
                            margin_left: "9.5rem",
                            width: "2rem",
                            color: "yellow",
                            "Cost (RM)"
                        }
                        a { margin_left: "4.5rem", width: "2rem", "Quantity" }
                        a { margin_left: "3.5rem", "Ordering" }
                    }

                    div {
                        id: "contained",
                        margin_bottom: "0.5rem",
                        flex_grow: 1,
                        flex_shrink: 0,
                        width: "stretch",
                        max_width: "100%",
                        max_height: "50vh",
                        overflow: "none",
                        overflow_y: "scroll",

                        for (item, indexed) in output().0 {
                            div {
                                class: "center",
                                justify_content: "flex-start",
                                margin: "2rem 0.8rem 2rem 0.8rem",

                                Tooltip {
                                    TooltipTrigger {
                                        img {
                                            src: item.img,
                                            border: "1px",
                                            border_style: "solid",
                                            padding: "10px",
                                            object_fit: "contain",
                                            width: "60rem",
                                            height: "60rem",
                                        }
                                    }
                                    TooltipContent {
                                        side: ContentSide::Right,
                                        style: "width: 200px",
                                        p { color: "black",
                                            {
                                                let all_sort: String = item
                                                    .tags
                                                    .iter()
                                                    .map(|i| i.to_string())
                                                    .collect::<Vec<_>>()
                                                    .join(", ");
                                                format!("{}", all_sort)
                                            }
                                        }
                                    }
                                }
                                p { flex_grow: 1, margin_left: "1rem", "{item.name}" }
                                p { margin: "0 5em 0 5em", width: "5rem", "{item.creator}" }
                                p {
                                    margin: "0 4em 0 4em",
                                    width: "2rem",
                                    color: "yellow",
                                    "{item.cost:0.2}"
                                }
                                p { margin: "0 4em 0 4em", width: "2rem", "{item.quantity}" }

                                Sliding_Take { indexed, max: item.quantity as f64 }

                            }
                        }

                    }

                    div { id: "contained", width: "stretch",
                        div { flex_direction: "row", margin_bottom: "1rem",
                            a { margin_bottom: "1rem", color: "cyan", "Cost: " }
                            a { margin_bottom: "1rem", color: "yellow", "RM {output().1:0.2}" }
                        }
                        button {
                            padding: "1rem",
                            background: "green",
                            onclick: move |_event| {
                                active_item
                                    .write()
                                    .0
                                    .iter_mut()
                                    .for_each(|in_item| {
                                        in_item.quantity = in_item.quantity.saturating_sub(in_item.taking);
                                        in_item.taking = 0;
                                    });
                            },
                            "Buy"
                        }
                        button {
                            padding: "1rem",
                            background: "red",
                            onclick: move |_event| {
                                active_item.write().0.iter_mut().for_each(|in_item| in_item.taking = 0);
                            },
                            "Cancel"
                        }
                    }
                }

            }
        }
    }
}

#[component]
fn Item_Tag() -> Element {
    let mut state = use_context::<Signal<FilterState>>();

    let items = ItemTags::iter().enumerate().map(|(i, it)| {
        let cased = ccase!(snake -> title, it.as_ref());
        rsx! {
            SelectOption::<ItemTags> { index: i, value: it, text_value: "{it}", "{it.emoji()} {cased}" }
        }
    });

    rsx! {
        p { margin_top: "1rem", "Item Tags (Multi)" }
        SelectMulti::<ItemTags> {
            default_values: vec![],
            width: "16rem",
            on_values_change: move |event: Vec<ItemTags>| {
                state.write().item_tags = event.clone();
            },
            SelectGroup { {items} }
        }
    }
}

#[component]
fn Sort_By_Tag() -> Element {
    let mut state = use_context::<Signal<FilterState>>();

    let sort_by = SortBy::iter().enumerate().map(|(i, s)| {
        let cased = ccase!(sentence , s.as_ref());
        rsx! {
            SelectOption::<Option<SortBy>> { index: i, value: s, text_value: "{cased}", "{s.emoji()} {cased}" }
        }
    });

    rsx! {

        p { margin_top: "1rem", "Sort By" }
        Select::<Option<SortBy>> {
            width: "15rem",
            default_value: Some(SortBy::default()),
            on_value_change: move |event: Option<Option<SortBy>>| {
                state.write().sort_by = event.flatten().unwrap();
            },
            SelectGroup { {sort_by} }
        }
    }
}

#[component]
pub fn Sliding_Take(indexed: usize, max: f64) -> Element {
    let mut current_value = use_context::<Signal<ActiveItem>>();

    let updated = use_memo(move || {
        let taking = current_value
            .read()
            .0
            .get(indexed)
            .and_then(|v| Some(v.taking))
            .unwrap_or_default();
        taking as f64
    });

    rsx! {
        div {
            display: "flex",
            flex_direction: "column",
            align_items: "center",

            // Display the current value
            div { style: "margin-bottom: 15px; font-size: 16px; font-weight: bold;",
                "{updated()}"
            }

            Slider {
                label: "Take Slider",
                horizontal: true,
                min: 0.0,
                max,
                step: 1.0,
                default_value: 0.0,
                value: updated(),
                on_value_change: move |value: f64| {
                    if let Some(item) = current_value.write().0.get_mut(indexed) {
                        item.taking = value as u16;
                    }
                },
            }
        }
    }
}
