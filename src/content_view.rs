pub mod countdown;

pub enum Content {
    Html(String),
    // Img(Vec<u8>),
}

pub trait ContentView {
    fn materialize(&self) -> Content;
}
