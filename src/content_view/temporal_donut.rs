use crate::content_view::{Content, get_html_template};

use super::ContentView;
use chrono::{DateTime, Days, Local, TimeZone};
use fun_html::{
    Element,
    attr::{self, style},
    elt,
};

const JAVASCRIPT_STRING: &str = r###"
    const width = 450,
        height = 450,
        margin = 40;

    // The radius of the pieplot is half the width or half the height (smallest one). I subtract a bit of margin.
    const radius = Math.min(width, height) / 2 - margin

    // append the svg object to the div called 'my_dataviz'
    const svg = d3.select("#my_dataviz")
        .append("svg")
        .attr("width", width)
        .attr("height", height)
        .append("g")
        .attr("transform", `translate(${width / 2}, ${height / 2})`);

    const data = { spend: __elapsed__, remaining: __remaining__ }

    // set the color scale
    const color = d3.scaleOrdinal()
        .range(d3.schemeSet2);

    // Compute the position of each group on the pie:
    const pie = d3.pie()
        .value(function (d) { return d[1] })
    const data_ready = pie(Object.entries(data))
    // Now I know that group A goes from 0 degrees to x degrees and so on.

    // shape helper to build arcs:
    const arcGenerator = d3.arc()
        .innerRadius(0)
        .outerRadius(radius)

    // Build the pie chart: Basically, each part of the pie is a path that we build using the arc function.
    svg
        .selectAll('mySlices')
        .data(data_ready)
        .join('path')
        .attr('d', arcGenerator)
        .attr('fill', 'white')
        .attr("stroke", "black")
        .style("stroke-width", "2px")
        .style("opacity", 0.7)

    // Now add the annotation. Use the centroid method to get the best coordinates
    svg
        .selectAll('mySlices')
        .data(data_ready)
        .join('text')
        .text(function (d) { return "" + d.data[0] })
        .attr("transform", function (d) { return `translate(${arcGenerator.centroid(d)})` })
        .style("text-anchor", "middle")
        .style("font-size", 17)"###;

fn get_temporal_donut_html_content_div(
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
) -> Vec<Element> {
    let total_days = (end_date - start_date).num_days();
    let elapsed_days = (Local::now() - start_date).num_days().clamp(0, total_days);
    let remaining_days = total_days - elapsed_days;

    let elapsed_percentage = (elapsed_days as f64 / total_days as f64) * 100.0;
    let script_string= format!("<script>{}</script>", JAVASCRIPT_STRING
        .replace("__elapsed__", &elapsed_days.to_string())
        .replace("__remaining__", &remaining_days.to_string()));

    vec![
        elt::script([attr::src("https://d3js.org/d3.v6.js")], ""),
        elt::div(
            [attr::style("float:left;")],
            [
                elt::p([], [elt::text("Temporal progress on PolicyCORE")]),
                elt::br([]),
                elt::br([]),
                elt::p([], elt::text("we are")),
                elt::p([], elt::text(format!("{:.2}%", elapsed_percentage))),
                elt::p([], elt::text("of the way there!")),
            ],
        ),
        elt::div([attr::style("float:right;"), attr::id("my_dataviz")], []),
        elt::raw_unsafe(
            script_string
        )
    ]
}

pub struct TemporalDonut {
    start_date: DateTime<Local>,
    end_date: DateTime<Local>,
}

impl TemporalDonut {
    pub fn new(start_date: DateTime<Local>, end_date: DateTime<Local>) -> Self {
        TemporalDonut {
            start_date,
            end_date,
        }
    }
}

impl ContentView for TemporalDonut {
    fn materialize(&self) -> Content {
        Content::Html(
            get_html_template()(get_temporal_donut_html_content_div(
                self.start_date,
                self.end_date,
            ))
            .to_string(),
        )
    }
}

#[cfg(test)]
mod temporal_donut_tests {
    use chrono::{Duration, Local};

    use crate::content_view::{temporal_donut::TemporalDonut, Content, ContentView};

    #[test]
    fn materialize_test() {
        let start_date = Local::now() - Duration::days(15);
        let end_date = start_date + Duration::days(30);
        let temporal_donut = TemporalDonut::new(start_date, end_date);
        let content = temporal_donut.materialize();
        assert!(match content {
            Content::Html(html) =>
                html.contains("<script>"),
            _ => false,
        });
    }
}