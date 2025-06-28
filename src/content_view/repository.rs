use std::{sync::RwLock};

use chrono::{DateTime, Local};
// use headless_chrome::protocol::cdp::Network::PrivateNetworkRequestPolicy;

use crate::content_view::{
    self,
    countdown::{self, Countdown},
};

pub struct DisplayContent {
    img: Vec<u8>,
    valid_until: DateTime<Local>,
}

impl DisplayContent {
    pub fn zero() -> Self {
        DisplayContent {
            img: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            valid_until: Local::now(),
        }
    }
}

pub struct Repository {
    content_url: String,
    views: countdown::Countdown,
    cache: RwLock<DisplayContent>,
}

impl Repository {
    pub fn new(content_url: String) -> Self {
        let cd = Countdown::new(String::from("popermo sandbox in"), 2025, 10, 1).unwrap();
        Repository {
            content_url,
            views: cd,
            cache: RwLock::new(DisplayContent::zero()),
        }
    }

    pub fn get_content_view(&self) -> &impl content_view::ContentView {
        &self.views
    }

    pub fn get_content(&self) -> Vec<u8> {
        println!("getting content from repository");
        self.cache.read().unwrap().img.clone()
    }

    pub fn update_content(&self) {
        println!("updating content in repository");
        let cache_lock = self.cache.read().unwrap();
        if cache_lock.valid_until > Local::now() {
            println!("content is still valid, no update needed");
            return;
        }
        drop(cache_lock); // Release the read lock before acquiring a write lock
        println!("content is outdated, updating now");
        let content_new = content_view::html_renderer::render(&self.content_url).unwrap();
        let mut cache_lock_write = self.cache.write().unwrap();
        cache_lock_write.img = content_new;
        cache_lock_write.valid_until = Local::now() + chrono::Duration::hours(4);
        drop(cache_lock_write); // Explicitly drop the write lock
        println!("content updated");
    }
}
