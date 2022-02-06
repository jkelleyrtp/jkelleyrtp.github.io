use crate::content::{ContentItem, ContentList, ContentPost};
use dioxus::prelude::*;

pub fn PortfolioPost(cx: Scope) -> Element {
    cx.render(rsx! {
        ContentPost {}
    })
}

pub fn PortfolioList(cx: Scope) -> Element {
    cx.render(rsx! {
        ContentList {
            header: cx.render(rsx!{
                div { class: "max-w-lg lg:max-w-2xl mx-auto mb-16 text-center",
                    span { class: "text-xs font-semibold text-indigo-500 uppercase",
                        "Jon's Portfolio"
                    }
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-bold font-heading",
                        "Showcase of work, projects, teams, and work"
                    }
                    p { class: "text-base leading-relaxed lg:text-l lg:leading-relaxed text-gray-500",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                    }
                }
            }),
            content: &[
                ContentItem {},
                ContentItem {},
                ContentItem {},
                ContentItem {},
            ]
        }
    })
}
