use fun_html::{
    Document, Element,
    attr::{self, style},
    elt,
};
pub mod countdown;
pub mod temporal_donut;

pub enum Content {
    Html(String),
    // Img(Vec<u8>),
}

pub trait ContentView {
    fn get_name(&self) -> String;
    fn materialize(&self) -> Content;
}

fn get_html_template() -> fn(Vec<Element>) -> Document {
    |content| {
        fun_html::html(
            [],
            [
                elt::head([], []),
                elt::body(
                    [style("font-family: courier,monospace;")],
                    elt::div(
                        [attr::id("content"), style("height:480px;width:800px")],
                        content,
                    ),
                ),
            ],
        )
    }
}
