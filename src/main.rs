use std::{str::FromStr, sync::Arc};

use tiny_http::{Response, Server};

use crate::repository::DisplayContent;

mod content_view;
mod graphics_util;
mod orchistrator;
mod renderers;
mod repository;

fn main() {
    let port = "1032";
    let content_part = "/content/";
    let content_url = format!("http://localhost:{port}{content_part}");
    let default_content = DisplayContent::new(
        include_bytes!("under_construction.png").to_vec(),
        chrono::Local::now(),
    );
    print!("starting server on port: {port} ");
    let srv = Server::http(format!("0.0.0.0:{port}")).unwrap();
    println!("done");
    let server = Arc::new(srv);
    let repository = Arc::new(repository::Repository::new(default_content));
    let orchistrator_repo = repository.clone();

    let orchistrator = Arc::new(orchistrator::Orchistrator::new(
        content_url,
        orchistrator_repo,
    ));

    let local_orchistrator = orchistrator.clone();
    std::thread::spawn(move || {
        local_orchistrator.run();
    });

    loop {
        let this_server = server.clone();
        println!("waiting for requests");
        let request = this_server.recv().unwrap();

        println!(
            "received request! method: {:?}, url: {:?}",
            request.method(),
            request.url()
        );

        let orchistrator_content = orchistrator.clone();
        let local_rep = repository.clone();
        match request.url() {
            "/" => std::thread::spawn(move || {
                let header = tiny_http::Header::from_str("content-type: image/png").unwrap();
                let mut res = Response::from_data(local_rep.get_content());
                res.add_header(header);
                request.respond(res).unwrap()
            }),
            "/content/" => std::thread::spawn(move || {
                let content = orchistrator_content.get_materialized_html();
                request.respond(Response::from_data(content)).unwrap()
            }),
            _ => std::thread::spawn(|| request.respond(Response::from_string("404")).unwrap()),
        };
    }
}
