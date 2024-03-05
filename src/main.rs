#![allow(non_snake_case)]
use dioxus::prelude::*;

fn main() {
    launch(app);
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavWrapper)]
    #[route("/")]
    Home,

    #[route("/blog")]
    BlogList,

    #[route("/blog/{id}")]
    BlogPost { id: String },

    #[route("/portfolio")]
    PortfolioList,

    #[route("/portfolio/{id}")]
    PortfolioPost { id: String },

    // #[route("/resume")]
    // Resume,
    #[route("/contact")]
    Contact,
}

fn app() -> Element {
    rsx! { Router::<Route> {} }
}

fn NavWrapper() -> Element {
    rsx! {
        nav::Navigation {}
        Outlet::<Route>{}
    }
}

pub use components::*;
mod components {
    pub mod about;
    pub use about::*;

    pub mod blog;
    pub use blog::*;

    pub mod contact;
    pub use contact::*;

    pub mod content;
    pub use content::*;

    pub mod home;
    pub use home::Home;

    pub mod nav;
    pub use nav::Navigation;

    pub mod portfolio;
    pub use portfolio::*;

    pub mod portfolioitem;
    pub use portfolioitem::*;

    pub mod resume;
    pub use resume::*;
}
pub mod blog_content;
