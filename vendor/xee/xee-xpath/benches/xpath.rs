use divan::{black_box, Bencher};

use xee_xpath::{Documents, Queries, Query};

fn main() {
    divan::main();
}

#[divan::bench]
fn range(bencher: Bencher) {
    let queries = Queries::default();
    let mut q = queries.sequence("1 to 1000").unwrap();

    let mut documents = Documents::new();

    bencher.bench_local(move || {
        black_box(&mut q)
            .execute_build_context(&mut documents, |_build| ())
            .unwrap();
    });
}

#[divan::bench]
fn string_concat(bencher: Bencher) {
    let queries = Queries::default();
    let mut q = queries.sequence("concat('foo', 'bar')").unwrap();

    let mut documents = Documents::new();

    bencher.bench_local(move || {
        black_box(&mut q)
            .execute_build_context(&mut documents, |_build| ())
            .unwrap();
    });
}

#[divan::bench]
fn string_value(bencher: Bencher) {
    let mut documents = Documents::new();
    let handle = documents
        .add_string_without_uri("<doc>Hello world</doc>")
        .unwrap();

    let queries = Queries::default();
    let mut q = queries.sequence("/doc/string()").unwrap();

    bencher.bench_local(move || {
        black_box(&mut q).execute(&mut documents, handle).unwrap();
    });
}

#[divan::bench]
fn large_map(bencher: Bencher) {
    let mut documents = Documents::new();

    let queries = Queries::default();
    let mut q = queries
        .sequence("map:keys(map:merge(for $n in 1 to 5000 return map:entry($n, $n+1)))")
        .unwrap();

    bencher.bench_local(move || {
        black_box(&mut q)
            .execute_build_context(&mut documents, |_build| ())
            .unwrap()
    });
}

#[divan::bench]
fn element_with_attribute(bencher: Bencher) {
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
    let mut q = queries.sequence("/doc/p[@state = 'even']").unwrap();
    bencher.bench_local(move || {
        black_box(&mut q).execute(&mut documents, handle).unwrap();
    });
}
