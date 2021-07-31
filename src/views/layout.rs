use maud::{html, Markup, DOCTYPE};

use super::ViewContext;

fn header(title: &str) -> Markup {
    html! {
        (DOCTYPE)
        head {
            meta charset="utf-8";
            title { (title) }
            link rel="stylesheet" type="text/css" href="/public/style.css";
        }
    }
}

fn navbar(ctx: &ViewContext) -> Markup {
    html! {
        nav.navbar.is-primary {
            .container {
                .navbar-brand {
                    a.navbar-item href="/" {
                        "🦄 Cabbit Stables"
                    }

                    a.navbar-burger data-target="navbar" {
                        span {}
                        span {}
                        span {}
                    }
                }

                #navbar.navbar-menu {
                    .navbar-start {
                        a.navbar-item href="/" {
                            "Overview"
                        }

                        a.navbar-item.is-active[ctx.path_starts_with("/horses")] href="/horses" {
                            "Horses"
                        }
                    }
                    .navbar-end {}
                }
            }
        }
    }
}

pub fn layout(ctx: &ViewContext, title: &str, body: Markup) -> Markup {
    html! {
        (header(title))
        body {
            (navbar(ctx))
            (body)
        }
    }
}

pub fn section(content: Markup) -> Markup {
    html! {
        section.section {
            .container {
                (content)
            }
        }
    }
}
