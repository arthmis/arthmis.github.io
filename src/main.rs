use actix_web::server;
use actix_web::{fs, App};

fn main() {
    let _server = server::new(|| {
        App::new()
            .handler(
                "/",
                fs::StaticFiles::new("frontend/")
                    .expect("couldn't find frontend folder")
                    .index_file("index.html")
            )
            .finish()
    })
    .bind("127.0.0.1:8000")
    .expect("couldn't bind to port")
    .run();
}
