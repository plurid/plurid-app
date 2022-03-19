use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};



static PORT: i8 = 8080;


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("a Rust app deployed on plurid.app")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!(
        format!("starting server on {}", PORT),
    );

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}