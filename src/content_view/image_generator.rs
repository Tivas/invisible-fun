// <link rel="stylesheet" href="https://unpkg.com/d3-milestones/build/d3-milestones.css">
// <script src="https://unpkg.com/d3-milestones/build/d3-milestones.min.js"></script>

// <div id="timeline"></div>

// <script>
//   milestones('#timeline')
//     .mapping({
//       'timestamp': 'year',
//       'text': 'title'
//     })
//     .parseTime('%Y')
//     .aggregateBy('year')
//     .render([
//       { year: 789, title: 'Vikings begin attacks on England.' },
//       { year: 842, title: 'Vikings found Dublin in Ireland.' }
//       ...
//       { year: 1050, title: 'The city of Oslo is founded in Norway.' },
//       { year: 1066, title: 'Battle of Hastings.' }
//     ]);
// </script>

use crate::content_view::{Content, get_html_template};

use super::ContentView;
use chrono::{DateTime, Days, Local, TimeZone};
use fun_html::{
    Element,
    attr::{self, style},
    elt,
};