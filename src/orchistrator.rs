use std::sync::{Arc, RwLock};

use chrono::TimeZone;
use image::flat::View;

use crate::content_view::countdown::Countdown;
use crate::content_view::temporal_donut::TemporalDonut;
use crate::content_view::{Content, ContentView};
use crate::renderers::html_renderer;
use crate::repository::{self, DisplayContent};

pub struct Orchistrator {
    materialized_html: RwLock<String>,
    content_url: String,
    repository: Arc<repository::Repository>,
}

impl Orchistrator {
    pub fn new(content_url: String, repository: Arc<repository::Repository>) -> Self {
        Orchistrator {
            materialized_html: RwLock::new(String::new()),
            content_url,
            repository,
        }
        // let view = Countdown::new(String::from("popermo PolicyCORE sandbox in"), 2025, 10, 1).unwrap();
    }

    pub fn get_materialized_html(&self) -> String {
        self.materialized_html.read().unwrap().clone()
    }

    pub fn run(&self) {
        loop {
            if self.repository.cache_outdated() {
                //choose
                let view = TemporalDonut::new(
                    chrono::Local.with_ymd_and_hms(2025, 4, 1, 0, 0, 0).unwrap(),
                    chrono::Local
                        .with_ymd_and_hms(2027, 5, 17, 1, 59, 59)
                        .unwrap(),
                );
                //generate
                let content = match view.materialize() {
                    Content::Html(html_content) => {
                        let mut write_lock = self.materialized_html.write().unwrap();
                        *write_lock = html_content;
                        drop(write_lock);

                        println!("rendering html content");
                        // render html content
                        html_renderer::render(&self.content_url).unwrap()
                    }
                };
                self.repository.update_content(DisplayContent::new(
                    content,
                    chrono::Local::now() + chrono::Duration::hours(4),
                ));
            }
            std::thread::sleep(std::time::Duration::from_secs(3600)); // Check every hour
        }
    }
}
