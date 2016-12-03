extern crate prown;

use prown::prown::Prown;

fn main() {
    Prown::parse("tests/pr03/.prown.toml").unwrap();
}
