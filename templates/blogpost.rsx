
fn component(cx: Scope) -> Element {
    cx.render(rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto mb-16",
                div { class: "max-w-xl lg:max-w-2xl mx-auto text-center",
                    div { class: "flex mb-2 items-center justify-center",
                        a { class: "text-xs text-indigo-500",
                            href: "#",
                            "Home"
                        }
                        icons::icon_0 {}
                        a { class: "text-xs text-indigo-500",
                            href: "#",
                            "Blog"
                        }
                        icons::icon_1 {}
                        a { class: "text-xs text-indigo-500",
                            href: "#",
                            "Article"
                        }
                    }
                    h2 { class: "mb-6 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Lorem ipsum dolor sit amet consectutar domor at elis"
                    }
                    div { class: "flex items-center justify-center",
                        div { class: "mr-6",
                            img { class: "w-16 h-16 object-cover rounded-full",
                                alt: "",
                                src: "plain-assets/images/indigo-500-avatar.png",
                            }
                        }
                        div { 
                            h3 { class: "text-2xl font-bold font-heading",
                                "Danny Bailey"
                            }
                            p { class: "text-lg text-indigo-500",
                                "February 26, 2021"
                            }
                        }
                    }
                }
            }
            div { class: "h-96 mb-12 lg:mb-16",
                img { class: "w-full h-full object-cover",
                    alt: "",
                    src: "plain-assets/images/indigo-600-horizontal.png",
                }
            }
            div { class: "container px-4 mx-auto",
                div { class: "max-w-2xl mx-auto",
                    p { class: "mb-6 lg:mb-8 text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "Building websites from wireframes that I had received. Some of those
        questions were:"
                    }
                    p { class: "mb-6 lg:mb-8 text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "These types of questions led me to miss numerous deadlines, and I wasted
        time and energy in back-and-forth communication. Sadly, this situation
        could have been avoided if the wireframes had provided enough detail."
                    }
                    p { class: "text-lg leading-loose lg:text-xl lg:leading-relaxed text-gray-500",
                        "Now that I am a UX designer, I notice that some designers tend to forget
        that wireframes are equally creative and technical. We are responsible
        for designing great ideas, but we are also responsible for creating
        product specifications. I admit that there can be so many details to
        remember that it’s easy to lose track. To save time and energy for
        myself, I gatheindigo all of my years of wireframing knowledge into a
        single checklist that I refer to throughout the process. And now I am
        sharing this knowledge with you, so that you can get back to being
        creative."
                    }
                }
            }
        }
    ))
}

mod icons {
	use super::*;
    pub(super) fn icon_0(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "w-3 h-3 mx-1 text-indigo-500",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                stroke: "currentColor",
                fill: "none",
                path { 
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                }
            }
		))
	}
    pub(super) fn icon_1(cx: Scope) -> Element {
        cx.render(rsx!(
            svg { class: "w-3 h-3 mx-1 text-indigo-500",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                path { 
                    stroke_linecap: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                    stroke_linejoin: "round",
                }
            }
		))
	}
}
