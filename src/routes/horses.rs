use actix_web::{
    get,
    web::{self, ServiceConfig},
    HttpRequest, Responder,
};
use maud::html;

use crate::{models::horse::Horse, views::{self, ViewContext, horse_info_page, horse_list}};

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(all_horses);
    cfg.service(
        web::resource("/{horse_id}")
            .name("view_horse")
            .to(view_horse),
    );
}

#[get("")]
async fn all_horses(ctx: ViewContext, req: HttpRequest) -> impl Responder {
    let horses = vec![
        Horse::new("Bella", "Westfalen", 1),
        Horse::new("Trotzki", "Irish Cob", 2),
        Horse::new("Pumpkin", "DRP", 3),
    ];

    views::layout(
        &ctx,
        "All Horses — Stable Software",
        html! {
            (views::section(html!{
                h1.title { "All Horses" }
                hr;
                (horse_list(&horses, Some(|horse: &Horse| {
                    views::button_group(vec![
                        html!{
                            a.button.is-link href=(req.url_for("view_horse", &[horse.id().to_string()]).expect("Could not generate URL")) {
                                "View"
                            }
                        }
                    ])
                })))
            }))
        },
    )
}

async fn view_horse(ctx: ViewContext) -> impl Responder {
    let horse = Horse::new("Bella", "Westfalen", 1);

    views::layout(
        &ctx,
        &format!("{} — Stable Software", horse.name()),
        html! {
            (horse_info_page(&horse))
        },
    )
}
