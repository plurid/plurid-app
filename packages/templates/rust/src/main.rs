use actix_web::{
    get,
    post,
    web,
    App,
    HttpResponse,
    HttpServer,
    Responder,
};

use std::net::{
    IpAddr,
    Ipv4Addr,
    SocketAddr,
};



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
    let socket: SocketAddr = SocketAddr::new(
        IpAddr::V4(
            Ipv4Addr::new(127, 0, 0, 1),
        ),
        8080,
    );

    let port = socket.port();
    println!("starting server on {}", port);

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((socket.ip(), port))?
    .run()
    .await
}
