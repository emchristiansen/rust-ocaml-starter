// Read the corresponding .ml file first.

use std::{time::Duration, thread};

ocaml::import! {
    fn hello_world() -> String;
}

#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub enum T {
    A,
    B(ocaml::Int),
}

ocaml::import! {
    fn maybe_inc(t: T) -> T;
}

// If the OCaml type `MyStruct.t` included a value we couldn't (or didn't want
// to) wrap, we would use `ocaml::Value` for it.
#[derive(ocaml::ToValue, ocaml::FromValue, Debug)]
pub struct MyStructT {
    a: ocaml::Float,
    b: ocaml::Int,
}

ocaml::import! {
    // We can optionally make input args references, allowing us to reuse a
    // value in Rust.
    // This does not require any changes to the OCaml code.
    fn mystruct_inc_both(t: &MyStructT) -> MyStructT;
}

// Run the non-async examples.
fn run(gc: &ocaml::Runtime) {
    unsafe {
        println!("hello_world: {}", hello_world(&gc).unwrap());
        println!("maybe_inc: {:?}", maybe_inc(&gc, T::B(1)).unwrap());
        println!(
            "mystruct_inc_both: {:?}",
            mystruct_inc_both(&gc, &MyStructT { a: 1.0, b: 2 }).unwrap()
        );
    }
}

// Run the async example.
async fn run_lwt(gc: &ocaml::Runtime) {
}

fn main() {
    let gc = ocaml::runtime::init();
    run(&gc);
    run_lwt(&gc).await;
}
