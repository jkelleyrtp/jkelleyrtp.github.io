use dioxus::prelude::*;

#[component]
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
                        a {
                            href: "https://dioxuslabs.com",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "Dioxus Labs, YC S23"
                        }
                    }
                }
                ul { class: "list-disc text-left pl-4 lg:pl-20 mb-4 text-gray-500 mx-auto max-w-screen-md",
                    li {
                        "Currently making crossplatform development easier with "
                        a {
                            href: "http://github.com/dioxuslabs",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "Dioxus"
                        }
                    }
                    li {
                        "Previously, Systems Engineer at "
                        a {
                            href: "https://cloudflare.com",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "Cloudflare"
                        }
                    }
                    li {
                        "Previously, Fullstack Engineer at "
                        a {
                            href: "http://hash.ai",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "HASH.ai"
                        }
                    }
                    li {
                        "Previously, Hardware Intern at "
                        a {
                            href: "https://www.nasa.gov/langley/",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "NASA Langley Research Center"
                        }
                    }
                    li {
                        "Graduated with bachelor's in ECE at "
                        a {
                            href: "http://olin.edu",
                            class: "font-bold text-blue-600",
                            target: "_blank",
                            "Olin College of Engineering"
                        }
                    }
                }
                img {
                    class: "mx-auto object-cover rounded-lg md:max-w-screen-md",
                    src: asset!("/public/images/profile_photo_2.jpg", AssetOptions::image().with_format(ImageFormat::Unknown)),
                    alt: ""
                }
            }
        }
    )
}
