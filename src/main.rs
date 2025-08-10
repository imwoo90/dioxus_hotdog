use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("assets/tailwind.css") },
        main {
            "data-theme": "cupcake",
            class: "h-screen flex flex-col",
            div {
                class: "navbar bg-base-100",
                width: "95%",
                margin: "auto",
                div {
                    class: "flex-1",
                    a {
                        class: "btn btn-ghost text-xl",
                        href: "#",
                        "daisyUI"
                    }
                }
                div {
                    class: "flex-none",
                    ul {
                        class: "menu menu-horizontal px-1",
                        li { a { href: "#", "Link" } }
                        li {
                            details {
                                summary { "Parent" }
                                ul {
                                    class: "p-2 bg-base-100 rounded-t-none",
                                    li { a { href: "#", "Link 1" } }
                                    li { a { href: "#", "Link 2" } }
                                }
                            }
                        }
                    }
                }
            }
            div {
                class: "hero flex-grow bg-base-200",
                div {
                    class: "hero-content text-center",
                    div {
                        class: "max-w-md",
                        h1 {
                            class: "text-5xl font-bold",
                            "Hello there"
                        }
                        p {
                            class: "py-6",
                            "Provident cupiditate voluptatem et in. Quaerat fugiat ut assumenda excepturi exercitationem quasi. In deleniti eaque aut repudiandae et a id nisi."
                        }
                        button {
                            class: "btn btn-primary",
                            "Get Started"
                        }
                    }
                }
            }
            footer {
                class: "footer footer-center p-4 bg-base-300 text-base-content",
                aside {
                    p { "Copyright © 2024 - All right reserved by ACME Industries Ltd" }
                }
            }
        }
    }
}