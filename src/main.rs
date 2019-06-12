pub enum Enum {
    A(Struct),
}

pub struct Struct {
    data: Enum,
}

fn main() {}
