
fn component() -> Element {
    cx.render(rsx!(
        section { class: "py-20",
            div { class: "container px-4 mx-auto",
                div { class: "max-w-xl lg:max-w-3xl mx-auto text-center",
                    span { class: "text-xs font-semibold text-indigo-500 uppercase",
                        "New Feature"
                    }
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-5xl lg:leading-tight font-bold font-heading",
                        "Jonathan Kelley"
                    }
                    p { class: "mb-8 text-base leading-relaxed lg:text-xl lg:leading-relaxed text-gray-500",
                        "Systems Engineer passionate about data, IoT,"
                    }
                    div { class: "mb-12",
                        a { class: "inline-block px-5 py-3 md:mr-3 mb-3 md:mb-0 text-sm bg-indigo-500 hover:bg-indigo-600 text-white font-semibold border border-indigo-500 hover:border-indigo-600 rounded transition duration-200",
                            href: "#",
                            "Try Demo"
                        }
                    }
                    img { class: "mx-auto object-cover rounded-lg",
                        src: "plain-assets/images/indigo-600-horizontal.png",
                        alt: "",
                    }
                }
            }
        }
    ))
}
