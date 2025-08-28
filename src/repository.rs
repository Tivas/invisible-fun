use chrono::{DateTime, Local};
use std::sync::RwLock;

use crate::graphics_util;

pub struct DisplayContent {
    img: Vec<u8>,
    valid_until: DateTime<Local>,
}

impl DisplayContent {
    pub fn new(img: Vec<u8>, valid_until: DateTime<Local>) -> Self {
        DisplayContent {
            img: graphics_util::resize_png_image(
                img,
                800,
                480,
            ),
            valid_until
        }
    }
}

pub struct Repository {
    cache: RwLock<DisplayContent>,
}

impl Repository {
    pub fn new(default_content:DisplayContent) -> Self {
        Repository {
            cache: RwLock::new(default_content),
        }
    }

    pub fn cache_outdated(&self) -> bool {
        let cache_lock = self.cache.read().unwrap();
        cache_lock.valid_until < Local::now()
    }

    pub fn get_content(&self) -> Vec<u8> {
        println!("getting content from repository");
        self.cache.read().unwrap().img.clone()
    }

    pub fn update_content(&self, display_content : DisplayContent) {
        *self.cache.write().unwrap() = display_content;
    }
}
