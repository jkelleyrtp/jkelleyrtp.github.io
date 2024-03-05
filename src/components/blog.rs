use crate::blog_content::BLOG_POSTS;
use crate::content::{ContentList, ContentPost};
use dioxus::prelude::*;

#[component]
pub fn BlogPost(id: String) -> Element {
    rsx! {
        ContentPost {
            post: BLOG_POSTS.iter().find(|p| p.title == id).unwrap(),
        }
    }
}

pub fn BlogList() -> Element {
    rsx! {
        div {
            ContentList {
                header: rsx!{
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
                },
                content: &BLOG_POSTS,
                readmore: true
            }
        }
    }
}
