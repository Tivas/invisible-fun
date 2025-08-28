use std::sync::{Arc, RwLock};

use crate::content_view::countdown::Countdown;
use crate::content_view::{Content, ContentView};
use crate::repository::{self, DisplayContent};
use crate::renderers::html_renderer;

pub struct Orchistrator {
    materialized_html: RwLock<String>,
    content_url: String,
    repository: Arc<repository::Repository>
}

impl Orchistrator {
    pub fn new(content_url: String, repository: Arc<repository::Repository>) -> Self {
        Orchistrator {
            materialized_html: RwLock::new(String::new()),
            content_url,
            repository,
        }
    }

    pub fn get_materialized_html(&self) -> String {
        self.materialized_html.read().unwrap().clone()
    }

    pub fn run(&self) {
          // println!("content is outdated, updating now");
        // let content_new = content_view::html_renderer::render(&self.content_url).unwrap();
        // let mut cache_lock_write = self.cache.write().unwrap();
        // cache_lock_write.img = content_new;
        // cache_lock_write.valid_until = Local::now() + chrono::Duration::hours(4);
        // drop(cache_lock_write); // Explicitly drop the write lock
        // println!("content updated");
        loop {
            if self.repository.cache_outdated() {
                //choose
                let view = Countdown::new(String::from("popermo PolicyCORE sandbox in"), 2025, 10, 1).unwrap();
                //generate
                let content  = match view.materialize() {
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

// pub fn orch_loop() {'a

        // println!("content is outdated, updating now");
        // let content_new = content_view::html_renderer::render(&self.content_url).unwrap();
        // let mut cache_lock_write = self.cache.write().unwrap();
        // cache_lock_write.img = content_new;
        // cache_lock_write.valid_until = Local::now() + chrono::Duration::hours(4);
        // drop(cache_lock_write); // Explicitly drop the write lock
        // println!("content updated");

// }