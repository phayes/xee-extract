use std::hint::black_box;

use xee_xpath::{Documents, Queries, Query};

fn main() {
    let mut documents = Documents::new();
    // large document doc with 1000 p elements where even elements have the
    // attribute state='even' and odd elements have the attribute state='odd'
    let mut doc = String::from("<doc>");
    for i in 0..1000 {
        if i % 2 == 0 {
            doc.push_str(&format!("<p state='even'>{}</p>", i));
        } else {
            doc.push_str(&format!("<p state='odd'>{}</p>", i));
        }
    }
    doc.push_str("</doc>");
    let handle = documents.add_string_without_uri(&doc).unwrap();
    let queries = Queries::default();
    let q = queries.sequence("/doc/p[@state = 'even']").unwrap();

    for _i in 0..1000 {
        let _r = black_box(q.execute(&mut documents, handle).unwrap());
    }
}
