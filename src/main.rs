#![allow(non_snake_case)]

use dioxus::prelude::*;
use dioxus::router::{Route, Router};

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            nav::Navigation {}
            Route { to: "/home", home::Home {} }
            Route { to: "/blog", "blog" }
            Route { to: "/blog/:post", "blog" }
            Route { to: "/portfolio", "portfolio" }
            Route { to: "/portfolio/:post", "portfolio" }
            Route { to: "/about", "about" }
            Route { to: "/resume", "resume" }
            Route { to: "", fallback: true, home::Home {} }
        }
    })
}

pub use components::*;
mod components {
    pub mod about;
    pub mod blogpost;
    pub mod blogposts;
    pub mod home;
    pub mod nav;
    pub mod portfolio;
    pub mod portfolioitem;
    pub mod resume;
}
