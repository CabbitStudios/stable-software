mod logging;
mod models;
mod routes;
mod views;

use actix_web::{get, middleware, web, App, HttpServer, Responder};
use maud::html;
use views::ViewContext;

#[get("/")]
async fn index(ctx: ViewContext) -> impl Responder {
    views::layout(
        &ctx,
        "Index — Stable Software",
        html! {
            (views::section(html!{
                h1.title { "Hello World!" }
                p { "Moshi Moshu!" }
            }))
        },
    )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let subscriber = logging::get_subscriber("app".into(), "info".into());
    logging::init_subscriber(subscriber);

    HttpServer::new(|| {
        let mut app = App::new()
            .wrap(tracing_actix_web::TracingLogger)
            .wrap(middleware::NormalizePath::new(
                middleware::normalize::TrailingSlash::Trim,
            ))
            .service(actix_files::Files::new("/public", "./public/"))
            .service(web::scope("/horses").configure(routes::horses::configure))
            .service(index);

        #[cfg(debug_assertions)]
        {
            app = app.service(actix_files::Files::new("/node_modules", "./node_modules/"));
        }

        app
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
