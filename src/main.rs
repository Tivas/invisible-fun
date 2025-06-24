use std::sync::Arc;

use tiny_http::{Response, Server};

use crate::content_view::{ContentView, countdown::Countdown};

mod content_view;
mod html_renderer;

fn main() {
    let port = "1032";
    let local_link = format!("http://localhost:{}", port);
    print!("starting server on port: {} ", port);
    let srv = Server::http(format!("0.0.0.0:{}", port)).unwrap();
    println!("done");
    let cd = Arc::new(Countdown::new(String::from("next sandbox in:"), 2025, 09, 01).unwrap());
    let server = Arc::new(srv);

    loop {
        let this_server = server.clone();
        println!("waiting for requests");
        let request = this_server.recv().unwrap();

        println!(
            "received request! method: {:?}, url: {:?}, headers: {:?}",
            request.method(),
            request.url(),
            request.headers()
        );

        let local_link_clone = local_link.clone();
        let countdown_clone: Arc<Countdown> = cd.clone();

        match request.url() {
            "/" => std::thread::spawn(move || {
                let route = format!("{}/content/", local_link_clone);
                let data = html_renderer::render(&route).unwrap();
                request.respond(Response::from_data(data)).unwrap()
            }),
            "/content/" => std::thread::spawn(move || {
                request
                    .respond(Response::from_data(countdown_clone.clone().to_html()))
                    .unwrap()
            }),
            _ => std::thread::spawn(|| request.respond(Response::from_string("404")).unwrap()),
        };
    }
}
