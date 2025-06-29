pub mod countdown;
pub mod html_renderer;
pub mod repository;

pub enum Content {
    Html(String),
    // Img(Vec<u8>),
}

pub trait ContentView {
    fn materialize(&self) -> Content;
}
