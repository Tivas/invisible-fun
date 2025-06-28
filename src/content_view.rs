pub mod countdown;
pub mod repository;
pub mod html_renderer;

pub enum Content {
    Html(String),
    // Img(Vec<u8>),
}

pub trait ContentView {
    fn materialize(&self) -> Content;
}
