use actix_web::{
    get, post,
    web::{self},
    App, HttpRequest, HttpServer, Responder,
};

use actix_cors::Cors;

use std::fs;

mod sites;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("ACTIX_WEB_ORIGIN", "*"); // <-- set allowed origin to *

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_headers(vec!["Authorization"])
            .allowed_methods(vec!["GET", "POST"]);

        let mut app = App::new().wrap(cors);

        app = app.service(web::resource("/photos").route(web::get().to(sites::photos)));

        return app;
    });

    server.bind(("0.0.0.0", 9001))?.run().await?;

    return Ok(());
}

fn get_photos() -> Vec<String> {
    let mut paths: Vec<String> = vec![];
    let files = fs::read_dir("./images").unwrap();
    for path in files {
        match path {
            Ok(n) => match n.file_name().to_str() {
                Some(n) => paths.push(n.to_owned()),
                None => (),
            },
            Err(_) => (),
        }
    }
    println!("Got request returning {:#?}", paths);
    return paths;
}
