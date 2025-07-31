use std::hint::black_box;

use xee_xpath::{Documents, Queries, Query};

fn main() {
    let mut documents = Documents::new();

    let queries = Queries::default();
    let q = queries
        .sequence("map:keys(map:merge(for $n in 1 to 500000 return map:entry($n, $n+1)))")
        .unwrap();

    for _i in 0..5 {
        let _r = black_box(
            q.execute_build_context(&mut documents, |_build| ())
                .unwrap(),
        );
    }
}
