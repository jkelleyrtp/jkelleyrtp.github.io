use crate::blog_content::PORTFOLIO_ENTRIES;
use crate::content::{ContentList, ContentPost};
use dioxus::prelude::*;

pub fn PortfolioPost(cx: Scope) -> Element {
    let id = dioxus_router::use_route(&cx).segment("post")?;

    let post = PORTFOLIO_ENTRIES.iter().find(|p| p.title == id)?;

    cx.render(rsx! {
        ContentPost { post: post }
    })
}

pub fn PortfolioList(cx: Scope) -> Element {
    cx.render(rsx! {
        ContentList {
            header: cx.render(rsx!{
                div { class: "max-w-lg lg:max-w-2xl mx-auto mb-16 text-center",
                    // span { class: "text-xs font-semibold text-indigo-500 uppercase",
                    //     "Jon's Portfolio"
                    // }
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-bold font-heading",
                        "Project Portfolio"
                    }
                    // p { class: "text-base leading-relaxed lg:text-l lg:leading-relaxed text-gray-500 italic",
                    //     "\"Motivation is a fire from within. If someone else tries to light that fire under you, chances are it will burn very briefly.\""
                    //     em { class: "font-bold", " - Stephen R. Covey" }
                    // }
                }
            }),
            content: &PORTFOLIO_ENTRIES,
            readmore: false
        }
    })
}
