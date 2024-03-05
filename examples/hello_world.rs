use std::fs::write;
use std::io;
use c_emit::Code;

fn main() -> io::Result<()> {
    let mut code = Code::new();

    code.include("stdio.h");
    code.call_func("printf");

    write("examples/hello_world.c", code.to_string())
}