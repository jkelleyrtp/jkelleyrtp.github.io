use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx!(
        section { class: "py-12",
            div { class: "container px-4 mx-auto",
                div { class: "max-w-xl lg:max-w-3xl mx-auto text-center",
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Jonathan Kelley"
                    }
                    p { class: "mb-4 text-base leading-relaxed lg:text-xl lg:leading-relaxed text-gray-500",
                        "Founder/CEO of "
                        a { href: "https://dioxuslabs.com", class: "font-bold text-blue-600", target: "_blank",
                            "Dioxus Labs, YC S23"
                            }
                        }
                    }
                    ul { class: "list-disc text-left pl-4 lg:pl-20 mb-4 text-gray-500 mx-auto max-w-screen-md",
                        li {
                            "Currently making crossplatform development easier with "
                            a { href: "http://github.com/dioxuslabs", class: "font-bold text-blue-600", target: "_blank",
                                "Dioxus"
                            }
                        }
                        li {
                            "Previously, Systems Engineer at "
                            a { href: "https://cloudflare.com", class: "font-bold text-blue-600", target: "_blank", "Cloudflare" }
                        }
                        li {
                            "Previously, Fullstack Engineer at "
                            a { href: "http://hash.ai", class: "font-bold text-blue-600", target: "_blank",
                                "HASH.ai"
                            }
                        }
                        li {
                            "Previously, Hardware Intern at "
                            a { href: "https://www.nasa.gov/langley/", class: "font-bold text-blue-600", target: "_blank",
                                "NASA Langley Research Center"
                            }
                        }
                        li {
                            "Graduated with bachelor's in ECE at "
                            a { href: "http://olin.edu", class: "font-bold text-blue-600", target: "_blank",
                                "Olin College of Engineering"
                            }
                        }
                        // li {
                        //     "Partner at student-led venture capital firm "
                        //     a { href: "https://www.248builders.vc", class: "font-bold text-blue-600", target: "_blank",
                        //         "248 Builders"
                        //     }
                        // }
                        // todo: re-enable when we have a website for leaf
                        // li {
                        //     "Solving asset management for manufacturing at "
                        //     a { href: "https://leafsystems.io", class: "font-bold text-blue-600", target: "_blank",
                        //         "Leaf Systems"
                        //     }
                        // }
                        // li {
                        //     "Passionate about wirless communications and "
                        //     a { href: "http://github.com/jkelleyrtp/ofdm", class: "font-bold text-blue-600", target: "_blank",
                        //         "implemented WiFi from scratch"
                        //     }
                        // }
                        // li {
                        //     "Published research on electrostatic nuclear fusion in "
                        //     a { href: "https://broadstreetscientific.ncssm.edu", class: "font-bold text-blue-600", target: "_blank",
                        //         "Broadstreet Scientific"
                        //     }
                        // }
                        // li {
                        //     "Co-founder and lead engineer of FIRST Robotics Team "
                        //     a { href: "https://cortechsrobotics.com", class: "font-bold text-blue-600", target: "_blank",
                        //         "Cortechs Robotics"
                        //     }
                        // }
                        // li {
                        //     "Passionate about complex systems, hardware, and low-level software systems"
                        // }
                    }
                    img { class: "mx-auto object-cover rounded-lg max-w-screen-md",
                        src: "images/profile_photo_2.jpg",
                        alt: "",
                    }
            }
        }
    )
}

fn Brief() -> Element {
    rsx!(
        div { class: "container px-4 mx-auto py-20",
            div { class: "flex flex-wrap -mx-4",
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
    )
}

mod icons {
    use super::*;
    pub(super) fn icon_0() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                path {
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        )
    }
    pub(super) fn icon_1() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                stroke: "currentColor",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                path {
                    d: "M9 5l7 7-7 7",
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                }
            }
        )
    }
    pub(super) fn icon_2() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                view_box: "0 0 24 24",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                stroke: "currentColor",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9 5l7 7-7 7",
                    stroke_width: "2",
                }
            }
        )
    }
    pub(super) fn icon_3() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "none",
                view_box: "0 0 24 24",
                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M9 5l7 7-7 7",
                }
            }
        )
    }
    pub(super) fn icon_4() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                view_box: "0 0 24 24",
                stroke: "currentColor",
                fill: "none",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    d: "M9 5l7 7-7 7",
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    stroke_width: "2",
                }
            }
        )
    }
    pub(super) fn icon_5() -> Element {
        rsx!(
            svg { class: "ml-1 w-5 h-4",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                path {
                    stroke_width: "2",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    d: "M9 5l7 7-7 7",
                }
            }
        )
    }
}
