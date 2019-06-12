To reproduce the ICE:

* Step 1: do `cargo check`
* Step 2: edit `src/main.rs`, change `Box<Enum>` to `Enum`
* Step 3: do `cargo check` again