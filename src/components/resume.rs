use dioxus::prelude::*;

pub fn Resume(cx: Scope) -> Element {
    cx.render(rsx! {
        section {
            class: "px-4 py-4 md:px-16 lg:px-64",
            div { class: "container",
                div { class: "max-w-lg lg:max-w-2xl mx-auto mb-16 text-center",
                    h2 { class: "mt-2 mb-4 text-3xl leading-tight md:text-4xl md:leading-tight lg:text-3xl lg:leading-tight font-bold font-heading",
                        "Résumé"
                    }
                    p { class: "text-base leading-relaxed lg:text-l lg:leading-relaxed text-gray-500",
                        a {
                            href: "/static/jonathan_kelley_resume.pdf",
                            target: "_blank",
                            class: "text-blue-500 hover:text-blue-700",
                            "Click here to download PDF"
                        }
                    }
                }
                iframe {
                    class: "lg:px-32",
                    src: "/static/jonathan_kelley_resume.pdf",
                    width: "100%",
                    height: "1200px",
                }
            }
        }
    })
}
