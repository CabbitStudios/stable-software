use maud::{html, Markup};

use crate::models::horse::Horse;

pub fn horse_list<F: for<'r> Fn(&'r Horse) -> Markup>(
    horses: &[Horse],
    per_horse: Option<F>,
) -> Markup {
    html! {
        @for horse in horses {
            .box {
                article.media {
                    .media-left {
                        figure.image."is-64x64" {
                            img src="/public/images/horse-placeholder.svg" alt=(format!("Image of {}", horse.name()));
                        }
                    }
                    .media-content {
                        .content {
                            p {
                                strong { (horse.name()) }
                                " "
                                small { (horse.breed()) }
                                br;

                                "This is a horse"

                                @if let Some(per_horse) = per_horse.as_ref() {
                                    (per_horse(&horse))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn horse_info_page(horse: &Horse) -> Markup {
    html! {
        .columns.is-gapless {
            .column."is-2".aside.hero.is-fullheight-desktop {
                .block.aside-info {
                    .block.aside-title {
                        h1.title { (horse.name()) }
                        h2.subtitle { (horse.breed()) }
                    }
                    .block.aside-center {
                        .figure {
                            .image."is-128x128".is-pulled-left.avatar {
                                img src="/public/images/horse-placeholder.svg" alt=(format!("Image of {}", horse.name()));
                            }
                        }
                    }
                }
            }
            .column {
                section.section {
                    h3.title { "Overview" }
                    table.table {
                        tbody {
                            tr {
                                th { "Weight" }
                                td { "450 kg" }
                            }
                            tr {
                                th { "Height" }
                                td { "162 cm" }
                            }
                        }
                    }
                }
                section.section {
                    h3.title { "Tasks to be done" }
                    ul {
                        li { "Go out" }
                        li { "Feed" }
                    }
                }
            }
        }
    }
}
