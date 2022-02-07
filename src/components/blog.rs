use crate::content::{ContentItem, ContentList, ContentPost};
use dioxus::prelude::*;

pub fn BlogPost(cx: Scope) -> Element {
    cx.render(rsx! {
        ContentPost {}
    })
}

static CONTENT: &[ContentItem] = &[
    ContentItem { archetype: "blog" },
    ContentItem { archetype: "blog" },
    ContentItem { archetype: "blog" },
    ContentItem { archetype: "blog" },
];

pub fn BlogList(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            ContentList {
                header: cx.render(rsx!{
                    div { class: "max-w-lg lg:max-w-2xl mx-auto mb-16 text-center",
                        span { class: "text-xs font-semibold text-indigo-500 uppercase",
                            "Jon's Blog"
                        }
                        h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-bold font-heading",
                            "Musings about Engineering, Science, and Life"
                        }
                        p { class: "text-base leading-relaxed lg:text-l lg:leading-relaxed text-gray-500 italic",
                            "\"Never let the fear of striking out keep you from playing the game\""
                            em { class: "font-bold", " - Babe Ruth" }
                        }
                    }
                }),
                content: CONTENT,
            }
        }
    })
}