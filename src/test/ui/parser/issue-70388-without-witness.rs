// run-rustfix
// This is for checking if we can apply suggestions as-is.

pub struct Foo(i32);

fn main() {
    let Foo(...) = Foo(0); //~ ERROR unexpected `...`
    let [_, ..., _] = [0, 1]; //~ ERROR unexpected `...`
}
