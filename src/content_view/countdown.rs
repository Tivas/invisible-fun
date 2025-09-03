use crate::content_view::{Content, get_html_template};

use super::ContentView;
use chrono::{DateTime, Days, Local, TimeZone};
use fun_html::{
    Element,
    attr::{self, style},
    elt,
};

fn get_countdown_html_content_div(header_text: String, body_text: String) -> Vec<Element> {
        vec![
            elt::div(
                [attr::style("align-self: center;")],
                [elt::h1(
                    [style("text-align: center;")],
                    [elt::text(header_text)],
                )],
            ),
            elt::h2(
                [style(
                    "display: grid;justify-content: center;align-items: center;",
                )],
                [elt::text(body_text)],
            )
        ]
}

pub struct Countdown {
    title: String,
    date: DateTime<Local>,
}

impl Countdown {
    pub fn new(title: String, year: i32, month: u32, day: u32) -> Option<Self> {
        let date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
        match date {
            chrono::offset::LocalResult::Single(d) => Some(Countdown { title, date: d }),
            _ => None,
        }
    }
    pub fn days_from_now(&self) -> i64 {
        let today = Local::now().checked_add_days(Days::new(1)).unwrap();
        (self.date - today).num_days()
    }
}

impl ContentView for Countdown {
    fn materialize(&self) -> Content {
        Content::Html(get_html_template()(get_countdown_html_content_div(
            self.title.clone(),
            format!("{} Days", self.days_from_now()),
        )).to_string())
    }
}

#[cfg(test)]
mod countdown_tests {
    use super::*;

    #[test]
    fn countdown_html_output() {
        let cd = Countdown::new("100 years".to_string(), 2100, 5, 21).unwrap();
        let _html_text = cd.materialize();
    }
}
