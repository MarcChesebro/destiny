#[macro_use] extern crate prettytable;

fn main() {
    let table = ptable!(["ABC", "DEFG", "HIJKLMN"],
                        ["foobar", "bar", "foo"],
                        ["foobar2", "bar2", "foo2"]);
}