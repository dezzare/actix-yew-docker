use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use yew::prelude::*;

// #[get("/")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello World!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Manual hello")
// }

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .service(hello)
//             .service(echo)
//             .route("/teste", web::get().to(manual_hello))
//     })
//     .bind(("127.0.0.1", 3000))?
//     .run()
//     .await
//
// }

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{"Hello World!"}</h1>
    }
}
