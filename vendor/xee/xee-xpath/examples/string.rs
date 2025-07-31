use std::hint::black_box;

use xee_xpath::{Documents, Queries, Query};

fn main() {
    let mut documents = Documents::new();
    let handle = documents
        .add_string_without_uri("<doc><p>Hello</p><p>world</p></doc>")
        .unwrap();

    let queries = Queries::default();
    let q = queries.sequence("/doc/string()").unwrap();

    for _i in 0..100000 {
        let _r = black_box(q.execute(&mut documents, handle).unwrap());
    }
}
