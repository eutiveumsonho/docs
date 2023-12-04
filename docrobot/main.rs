mod parsers;
use parsers::links::update_links;

fn main() {
    update_links("./../eutiveumsonho/0.1.0", "/eutiveumsonho/0.1.0/").unwrap();
}