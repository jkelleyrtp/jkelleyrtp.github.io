#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::router::{Route, Router};

pub use components::*;
mod components {
    pub mod about;
    pub mod blog;
    pub mod contact;
    pub mod content;
    pub mod home;
    pub mod nav;
    pub mod portfolio;
    pub mod portfolioitem;
    pub mod resume;
}
pub mod blog_content;

fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            nav::Navigation {}
            Route { to: "/home", home::Home {} }
            Route { to: "/blog", blog::BlogList {} }
            Route { to: "/blog/:post", blog::BlogPost {} }
            Route { to: "/portfolio", portfolio::PortfolioList {} }
            Route { to: "/portfolio/:post", portfolio::PortfolioPost {} }
            Route { to: "/resume", resume::Resume {} }
            Route { to: "/contact", contact::Contact {} }
            Route { to: "", home::Home {} }
        }
    })
}
