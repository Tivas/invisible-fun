pub mod countdown;

pub trait ContentView {
    fn to_html(&self) -> String;
}
