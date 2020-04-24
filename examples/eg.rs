use pmre::regex;

#[regex]
const x: i32 = r"\d{4}-\d{2}-\d{2}";

// #[regex]
// const y: i32 = r"(\d{4})-(\d{2})-(\d{2})";

fn main() {
    assert!(x.is_match(r"2014-01-01".as_bytes()))
}
