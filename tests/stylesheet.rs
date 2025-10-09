#![forbid(unsafe_code)]

use std::io::Cursor;

use xml::EventReader;
use xml::reader::XmlEvent;

macro_rules! assert_match {
    ($actual:expr, $expected:pat) => {
        match $actual {
            $expected => {},
            _ => panic!("assertion failed: `(left matches right)` \
                        (left: `{:?}`, right: `{}`", $actual, stringify!($expected))
        }
    };
    ($actual:expr, $expected:pat if $guard:expr) => {
        match $actual {
            $expected if $guard => {},
            _ => panic!("assertion failed: `(left matches right)` \
                        (left: `{:?}`, right: `{} if {}`",
                        $actual, stringify!($expected), stringify!($guard))
        }
    };
}

#[test]
fn stylesheet_pi_escaping() {

    let source = r#"<?xml version="1.0" standalone="no"?>
        <!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.0//EN"
        "http://www.w3.org/TR/2001/REC-SVG-20010904/DTD/svg10.dtd">
        <?xml-stylesheet type="text/css" href="../resources/test.css" ?>
        "#;


    let buf = Cursor::new(source);
    let reader = EventReader::new(buf);

    let mut it = reader.into_iter();

    assert_match!(it.next(), Some(Ok(XmlEvent::StartDocument { .. })));
    assert_match!(it.next(), Some(Ok(XmlEvent::Doctype { .. })));
    let pi = it.next();
    println!("{:?}", pi);
    assert_match!(pi, Some(Ok(XmlEvent::ProcessingInstruction { name, data })) if name == "xml-stylesheet" && data.as_deref() == Some(r#"type="text/css" href="../resources/test.css""#));
}
