extern crate handlebars_iron as hbsi;

use std::error::Error;
use hbsi::{HandlebarsEngine, DirectorySource};

#[test]
fn test_template() {
    let mut hbse = HandlebarsEngine::new2();
    let src = Box::new(DirectorySource::new("./examples/templates/", ".hbs"));

    hbse.add(src);

    // success of panic
    if let Err(r) = hbse.reload() {
        panic!("{}", r.description());
    }

    let hh = hbse.registry.read().unwrap();

    assert!(hh.get_template("index").is_some());
}
