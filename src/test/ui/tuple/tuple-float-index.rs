// run-rustfix
// compile-flags: -Z parse-only

fn main () {
    (1, (2, 3)).1.1; //~ ERROR unexpected token: `1.1`
}
