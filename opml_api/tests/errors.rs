use opml::*;
use std::fs::read_to_string as read;

#[test]
#[should_panic]
fn test_invalid_xml() {
  let sample = read("tests/samples/invalid_xml.opml").unwrap();
  OPML::new(sample.as_str()).unwrap();
}

#[test]
fn test_invalid_opml_version() {
  let sample = read("tests/samples/invalid_opml_version.opml").unwrap();
  let res = OPML::new(sample.as_str());
  assert!(matches!(res, Err(Error::UnsupportedVersion(e)) if e == "invalid"));
}

#[test]
fn test_invalid_opml_no_outlines() {
  let sample = read("tests/samples/invalid_opml_no_outlines.opml").unwrap();
  let res = OPML::new(sample.as_str());
  assert!(matches!(res, Err(Error::BodyHasNoOutlines)));
}
