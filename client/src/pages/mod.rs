use dioxus::prelude::*;
use route::Route;

mod blog;
mod home;
mod route;

pub fn app() -> Element {
    rsx! { Router::<Route> {} }
}
