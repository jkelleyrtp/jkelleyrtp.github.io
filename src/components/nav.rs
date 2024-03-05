use dioxus::prelude::*;
use dioxus_router::prelude::Link;

use crate::Route;

static LINKS: &[(&str, &str)] = &[
    // ("Home", "/"),
    // ("Portfolio", "portfolio"),
    // ("Blog", "blog"),
    // ("Résumé", "resume"),
    // ("Contact", "contact"),
];

fn LinkList() -> Element {
    rsx! {
        for (name, route) in LINKS.iter() {
            li {
                Link {
                    to: "{route}",
                    class: "text-sm hover:text-indigo-700 font-medium",
                    "{name}"
                }
            }
        }
        li {
            a {
                href: "https://www.linkedin.com/in/jonathan-r-kelley",
                target: "_blank",
                class: "text-sm hover:text-indigo-700 font-medium",
                icons::LinkedinLogo {}
            }
        }
        li {
            a {
                href: "https://github.com/jkelleyrtp",
                target: "_blank",
                class: "text-sm hover:text-indigo-700 font-medium",
                icons::GithubLogo {}
            }
        }
    }
}

pub fn Navigation() -> Element {
    let show = use_signal(|| false);
    rsx!(
        nav { class: "py-8 bg-transparent",
            MobileNav { show: show }
            FullNav { show: show }
        }
    )
}

#[component]
fn MobileNav(show: Signal<bool>) -> Element {
    rsx! {
        div { class: "container px-4 mx-auto",
            div { class: "flex justify-between items-center",
                Link {
                    to: Route::Home,
                    class: "text-gray-600 text-2xl leading-none",
                    "Jonathan Kelley"
                }
                div { class: "lg:hidden",
                    button { class: "block navbar-burger text-indigo-500 hover:text-indigo-700 focus:outline-none",
                        onclick: move |_| show.toggle(),
                        icons::icon_1 {}
                    }
                }
                ul { class: "hidden lg:flex ml-auto mr-10 items-center w-auto space-x-12",
                    LinkList {}
                }
            }
        }
    }
}

#[component]
fn FullNav(show: Signal<bool>) -> Element {
    rsx! {
        div {
            class: "navbar-menu fixed top-0 left-0 bottom-0 w-5/6 max-w-sm z-50",
            class: if show() { "" } else { "hidden" },
            div { class: "navbar-backdrop fixed inset-0 bg-gray-800 opacity-25", }
            nav { class: "relative flex flex-col py-6 px-6 w-full h-full bg-white border-r overflow-y-auto",
                div { class: "flex items-center mb-12",
                    a { class: "mr-auto text-2xl font-semibold leading-none",
                        href: "#",
                        img { class: "h-8",
                            width: "auto",
                            src: "plain-assets/logos/plain-indigo.svg",
                            alt: "",
                        }
                    }
                    button {
                        class: "navbar-close",
                        onclick: move |_| show.toggle(),
                        icons::icon_0 {}
                    }
                }
                div { ul { LinkList {} } }
            }
        }
    }
}

pub mod icons {
    use super::*;
    pub(super) fn icon_0() -> Element {
        rsx!(
            svg { class: "h-6 w-6 cursor-pointer hover:text-indigo-500",
                fill: "none",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                stroke: "currentColor",
                path {
                    stroke_width: "2",
                    stroke_linejoin: "round",
                    stroke_linecap: "round",
                    d: "M6 18L18 6M6 6l12 12",
                }
            }
        )
    }
    pub(super) fn icon_1() -> Element {
        rsx!(
            svg { class: "h-4 w-4",
                view_box: "0 0 20 20",
                fill: "currentColor",
                xmlns: "http://www.w3.org/2000/svg",
                title {
                    "Mobile menu"
                }
                path {
                    d: "M0 3h20v2H0V3zm0 6h20v2H0V9zm0 6h20v2H0v-2z",
                }
            }
        )
    }

    pub static GithubLogo: Component<()> = |cx| {
        rsx!(
            svg {
                height: "24",
                width: "24",
                fill: "currentColor",
                view_box: "0 0 16 16",
                path {
                    fill_rule: "evenodd",
                    d: "M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z",
                }
            }
        )
    };

    pub static LinkedinLogo: Component<()> = |cx| {
        rsx!(
            svg {
                height: "24",
                width: "24",
                fill: "currentColor",
                view_box: "0 0 24 24",
                path {
                    d: "M19 0h-14c-2.761 0-5 2.239-5 5v14c0 2.761 2.239 5 5 5h14c2.762 0 5-2.239 5-5v-14c0-2.761-2.238-5-5-5zm-11 19h-3v-11h3v11zm-1.5-12.268c-.966 0-1.75-.79-1.75-1.764s.784-1.764 1.75-1.764 1.75.79 1.75 1.764-.783 1.764-1.75 1.764zm13.5 12.268h-3v-5.604c0-3.368-4-3.113-4 0v5.604h-3v-11h3v1.765c1.396-2.586 7-2.777 7 2.476v6.759z",
                }
            }
        )
    };

    pub(super) fn EmailIcon() -> Element {
        rsx!(
            svg { class: "h-6 w-6",
                xmlns: "http://www.w3.org/2000/svg",
                fill: "currentColor",
                view_box: "0 0 20 20",
                stroke: "currentColor",
                path {
                    d: "M2.003 5.884L10 9.882l7.997-3.998A2 2 0 0016 4H4a2 2 0 00-1.997 1.884z",
                }
                path {
                    d: "M18 8.118l-8 4-8-4V14a2 2 0 002 2h12a2 2 0 002-2V8.118z",
                }
            }
        )
    }
}
