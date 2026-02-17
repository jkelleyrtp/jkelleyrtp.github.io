use dioxus::prelude::*;

pub fn Home() -> Element {
    rsx!(
        section { class: "py-20 lg:py-32 bg-gradient-to-b from-gray-50 to-white",
            div { class: "container px-4 mx-auto",
                div { class: "max-w-4xl mx-auto",
                    div { class: "flex flex-col lg:flex-row items-center gap-12 lg:gap-16",
                        div { class: "relative flex-shrink-0",
                            div { class: "absolute -inset-1 bg-gradient-to-r from-blue-600 to-indigo-600 rounded-full blur-md opacity-30" }
                            img {
                                class: "relative object-cover rounded-full ring-4 ring-white shadow-xl",
                                style: "width: 224px; height: 224px;",
                                src: asset!("/assets/images/profile_photo_2.jpg"),
                                alt: "Jonathan Kelley"
                            }
                        }
                        div { class: "text-center lg:text-left",
                            h1 { class: "text-4xl md:text-5xl lg:text-6xl font-extrabold tracking-tight",
                                span { class: "bg-gradient-to-r from-gray-900 via-gray-800 to-gray-600 bg-clip-text text-transparent",
                                    "Jonathan Kelley"
                                }
                            }
                            p { class: "mt-4 text-lg lg:text-xl text-gray-500",
                                "Founder/CEO of "
                                a {
                                    href: "https://dioxuslabs.com",
                                    class: "font-semibold text-blue-600 hover:text-blue-700 transition-colors duration-200",
                                    target: "_blank",
                                    "Dioxus Labs"
                                }
                                span { class: "text-gray-400 mx-1", " · " }
                                span { class: "text-orange-500 font-medium", "YC S23" }
                            }
                            div { class: "mt-8 flex flex-col gap-3",
                                ExperienceItem {
                                    text: "Making crossplatform development easier with ",
                                    link: "http://github.com/dioxuslabs",
                                    link_text: "Dioxus",
                                    current: true
                                }
                                ExperienceItem {
                                    text: "Previously, Systems Engineer at ",
                                    link: "https://cloudflare.com",
                                    link_text: "Cloudflare",
                                    current: false
                                }
                                ExperienceItem {
                                    text: "Previously, Fullstack Engineer at ",
                                    link: "http://hash.ai",
                                    link_text: "HASH.ai",
                                    current: false
                                }
                                ExperienceItem {
                                    text: "Previously, Hardware Intern at ",
                                    link: "https://www.nasa.gov/langley/",
                                    link_text: "NASA Langley",
                                    current: false
                                }
                                ExperienceItem {
                                    text: "Bachelor's in ECE from ",
                                    link: "http://olin.edu",
                                    link_text: "Olin College of Engineering",
                                    current: false
                                }
                            }
                        }
                    }
                }
            }
        }
        Footer {}
    )
}

#[component]
fn ExperienceItem(text: &'static str, link: &'static str, link_text: &'static str, current: bool) -> Element {
    rsx!(
        div { class: "flex items-start gap-3 text-left",
            div { class: "mt-1.5 flex-shrink-0",
                if current {
                    div { class: "w-2 h-2 rounded-full bg-green-500 ring-2 ring-green-200" }
                } else {
                    div { class: "w-2 h-2 rounded-full bg-gray-300" }
                }
            }
            p { class: "text-gray-600",
                "{text}"
                a {
                    href: "{link}",
                    class: "font-semibold text-gray-900 hover:text-blue-600 transition-colors duration-200",
                    target: "_blank",
                    "{link_text}"
                }
            }
        }
    )
}

fn Footer() -> Element {
    rsx!(
        footer { class: "py-12 bg-gray-50 border-t border-gray-100",
            div { class: "container px-4 mx-auto",
                div { class: "flex flex-col md:flex-row items-center justify-between gap-4",
                    p { class: "text-sm text-gray-400",
                        "Built with "
                        a {
                            href: "https://dioxuslabs.com",
                            class: "text-gray-500 hover:text-gray-700 transition-colors duration-200",
                            target: "_blank",
                            "Dioxus"
                        }
                    }
                    div { class: "flex items-center gap-6",
                        a {
                            href: "https://github.com/jkelleyrtp",
                            target: "_blank",
                            class: "text-gray-400 hover:text-gray-600 transition-colors duration-200",
                            crate::nav::icons::GithubLogo {}
                        }
                        a {
                            href: "https://www.linkedin.com/in/jonathan-r-kelley",
                            target: "_blank",
                            class: "text-gray-400 hover:text-gray-600 transition-colors duration-200",
                            crate::nav::icons::LinkedinLogo {}
                        }
                    }
                }
            }
        }
    )
}
