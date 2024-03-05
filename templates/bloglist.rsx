
fn component() -> Element {
    cx.render(rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto",
                div { class: "max-w-lg lg:max-w-2xl mx-auto mb-16 text-center",
                    span { class: "text-xs font-semibold text-indigo-500 uppercase",
                        "Lorem ipsum"
                    }
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Lorem ipsum dolor sit amet consectutar domor at elis"
                    }
                    p { class: "text-base leading-relaxed lg:text-xl lg:leading-relaxed text-gray-500",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                    }
                }
                div { class: "flex flex-wrap -mx-4",
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        alt: "",
                                        src: "plain-assets/images/indigo-600-square.png",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_0 {}
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        src: "plain-assets/images/indigo-600-square.png",
                                        alt: "",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_1 {}
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        src: "plain-assets/images/indigo-600-square.png",
                                        alt: "",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_2 {}
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        src: "plain-assets/images/indigo-600-square.png",
                                        alt: "",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_3 {}
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        src: "plain-assets/images/indigo-600-square.png",
                                        alt: "",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_4 {}
                                    }
                                }
                            }
                        }
                    }
                    div { class: "w-full md:w-1/2 px-4 mb-12 lg:mb-16",
                        div { class: "flex flex-wrap -mx-4",
                            div { class: "w-full lg:w-1/3 px-4 mb-6 lg:mb-0",
                                div { class: "h-80 lg:h-40",
                                    img { class: "w-full h-full object-cover rounded-lg",
                                        src: "plain-assets/images/indigo-600-square.png",
                                        alt: "",
                                    }
                                }
                            }
                            div { class: "w-full lg:w-2/3 px-4",
                                span { class: "text-xs font-bold text-indigo-500",
                                    "10 jun 2022 19:40"
                                }
                                h2 { class: "mt-2 mb-2 text-3xl font-bold font-heading",
                                    "Lorem ipsum dolor sit"
                                }
                                p { class: "mb-4 text-lg text-gray-500 leading-loose",
                                    "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Pellentesque massa nibh, pulvinar vitae aliquet nec, accumsan aliquet orci."
                                }
                                a { class: "flex items-center text-lg font-bold text-indigo-500 hover:text-indigo-700",
                                    href: "#",
                                    span {
                                        "Read more"
                                    }
                                    span {
                                        icons::icon_5 {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    ))
}

mod icons {
	use super::*;
    pub(super) fn icon_0() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9 5l7 7-7 7",
                    stroke_width: "2",
                }
            }
		))
	}
    pub(super) fn icon_1() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                fill: "none",
                path {
                    d: "M9 5l7 7-7 7",
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    stroke_width: "2",
                }
            }
		))
	}
    pub(super) fn icon_2() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                path {
                    stroke_width: "2",
                    stroke_linecap: "round",
                    d: "M9 5l7 7-7 7",
                    stroke_linejoin: "round",
                }
            }
		))
	}
    pub(super) fn icon_3() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                fill: "none",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                path {
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                    stroke_linecap: "round",
                }
            }
		))
	}
    pub(super) fn icon_4() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                stroke: "currentColor",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9 5l7 7-7 7",
                    stroke_width: "2",
                }
            }
		))
	}
    pub(super) fn icon_5() -> Element {
        cx.render(rsx!(
            svg { class: "ml-1 w-5 h-4",
                xmlns: "http://www.w3.org/2000/svg",
                stroke: "currentColor",
                fill: "none",
                view_box: "0 0 24 24",
                path {
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9 5l7 7-7 7",
                }
            }
		))
	}
}
