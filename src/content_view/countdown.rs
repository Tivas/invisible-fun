use crate::content_view::Content;

use super::ContentView;
use chrono::{DateTime, Days, Local, TimeZone};
use fun_html::{
    attr::{self, style},
    elt,
};

fn get_countdown_template() -> String {
    fun_html::html(
        [],
        [
            elt::head([], []),
            elt::body(
                [style("font-family: courier,monospace;")],
                elt::div(
                    [attr::id("content"), style("display:grid;height:480px;width:800px")],
                    [
                        elt::div(
                            [attr::style("align-self: center;")],
                            [elt::h1(
                                [style("text-align: center;")],
                                [elt::text("text1")],
                            )],
                        ),
                        elt::h2(
                            [style(
                                "display: grid;justify-content: center;align-items: center;",
                            )],
                            [elt::text("text2")],
                        ),
                    ],
                ),
            ),
        ],
    )
    .to_string()
}

pub struct Countdown {
    title: String,
    date: DateTime<Local>,
}

impl Countdown {
    pub fn new(title: String, year: i32, month: u32, day: u32) -> Option<Self> {
        let date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
        match date {
            chrono::offset::LocalResult::Single(d) => Some(Countdown {
                title,
                date: d,
            }),
            _ => None,
        }
    }
    pub fn days_from_now(&self) -> i64 {
        let today = Local::now().checked_add_days(Days::new(1)).unwrap();
        (self.date - today).num_days()
    }
}

impl ContentView for Countdown {
    fn materialize (&self) -> Content{
        let template_text = get_countdown_template();
        let data = template_text
            .replace("text1", &self.title)
            .replace("text2", &format!("{} Days", self.days_from_now()));
        Content::Html(data)
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
