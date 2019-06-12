pub enum Enum {
    A(Struct),
}

pub struct Struct {
    // To reproduce the ICE bug:
    // * Step 1: do `cargo check`
    // * Step 2: change `Box<Enum>` to `Enum`
    // * Step 3: do `cargo check` again
    data: Box<Enum>,
}

fn main() {}
