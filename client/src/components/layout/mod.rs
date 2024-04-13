use dioxus::prelude::*;

#[derive(PartialEq, Clone, Props)]
pub struct LayoutProps {
    children: Element,
}

#[component]
pub fn Layout(props: LayoutProps) -> Element {
    rsx! {
        div { class: "layout",
            header {
                h1 { "Layout Header" }
                time { "13.4.2024" }
            }
            nav { "this is nav" }
            aside { "this is aside" }
            main { {props.children} }
            footer { p { "© 2024 RR" } }
        }
    }
}
