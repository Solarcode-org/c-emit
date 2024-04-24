use std::fs::write;
use std::io;

use c_emit::{CArg, Code, VarInit};

fn main() -> io::Result<()> {
    let mut code = Code::new();

    code.include("stdio.h");
    code.call_func_with_args("printf", vec![CArg::String("Hello World!")]);
    code.new_var("a", VarInit::SizeString(5));
    code.call_func_with_args("scanf", vec![CArg::String("%s"), CArg::Ident("a")]);

    code.exit(1);

    write("examples/hello_world.c", code.to_string())
}
