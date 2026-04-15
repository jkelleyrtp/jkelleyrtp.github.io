use dioxus::prelude::*;

fn main() {
    //     <title>Jonathan Kelley's Portfolio</title>
    //     <link rel="stylesheet" type="text/css" href="https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/5.0.0/github-markdown-light.min.css">
    //     <meta name="viewport" content="width=device-width, initial-scale=1.0">
    //     <style>
    //   .markdown-body {
    //     box-sizing: border-box;
    //     min-width: 200px;
    //     max-width: 980px;
    //     margin: 0 auto;
    //     padding: 45px;
    //     list-style: disc;
    //   }

    //   @media (max-width: 767px) {
    //     .markdown-body {
    //       padding: 15px;
    //     }
    //   }

    dioxus::launch(|| {
        rsx! {
            Router::<Route> {}
        }
    });
}

#[derive(Routable, PartialEq, Clone)]
enum Route {
    #[layout(NavWrapper)]
    #[route("/")]
    Home,

    #[route("/blog")]
    BlogList,

    #[route("/blog/:id")]
    BlogPost { id: String },

    #[route("/portfolio")]
    PortfolioList,

    #[route("/portfolio/:id")]
    PortfolioPost { id: String },

    #[route("/resume")]
    Resume,

    #[route("/contact")]
    Contact,
}

#[component]
fn NavWrapper() -> Element {
    rsx! {
        nav::Navigation {}
        link { rel: "stylesheet", href: asset!("/assets/tailwind.css") }
        Outlet::<Route> {}
    }
}

pub use components::*;
mod components {

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

    pub mod resume;
    pub use resume::*;
}
pub mod blog_content;
