use std::{str::FromStr, sync::Arc};

use tiny_http::{Response, Server};

use crate::content_view::{ContentView, repository};

mod content_view;
mod graphics_util;

fn main() {
    let port = "1032";
    let content_part = "/content/";
    let content_url = format!("http://localhost:{port}{content_part}");
    print!("starting server on port: {port} ");
    let srv = Server::http(format!("0.0.0.0:{port}")).unwrap();
    println!("done");
    let server = Arc::new(srv);
    let repository = Arc::new(repository::Repository::new(content_url.clone()));

    loop {
        let this_server = server.clone();
        println!("waiting for requests");
        let request = this_server.recv().unwrap();

        println!(
            "received request! method: {:?}, url: {:?}",
            request.method(),
            request.url()
        );

        let local_rep = repository.clone();
        match request.url() {
            "/" => {
                std::thread::spawn(move || {
                    let header = tiny_http::Header::from_str("content-type: image/png")
                        .unwrap();
                    let mut res = Response::from_data(local_rep.get_content());
                    res.add_header(header);
                    request
                        .respond(res)
                        .unwrap()
                });
                let local_rep_update = repository.clone();
                std::thread::spawn(move || local_rep_update.update_content())
            }
            "/content/" => std::thread::spawn(move || {
                let crate::content_view::Content::Html(content) =
                    local_rep.get_content_view().materialize();
                request.respond(Response::from_data(content)).unwrap()
            }),
            _ => std::thread::spawn(|| request.respond(Response::from_string("404")).unwrap()),
        };
    }
}
