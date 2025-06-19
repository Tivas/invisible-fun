mod webpage_renderer;
fn main() {
    // webpage_renderer::render_page("./src/test.htm");
    webpage_renderer::screenshot_thepage().expect("Failed to take screenshot");
    println!("Hello, world!");
}
