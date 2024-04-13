use crate::components::layout::Layout;
use crate::pages::route::Route;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        Layout {
            Link { to: Route::Blog { id: count() }, "Go to blog" }
            div {
                h1 { "High-Five counter: {count}" }
                button { onclick: move |_| count += 1, "Up high!" }
                button { onclick: move |_| count -= 1, "Down low!" }
            }
        }
    }
}
