use std::env;
use std::fs::File;
use std::io::prelude::*;

use crate::asm_generator::generate_asm;

mod ast;
mod lexxer;
mod asm_generator;




fn main(){
    let args: Vec<String> = env::args().collect();
    let mut lexxed_file = lexxer::lex(&args[1]);
    
    let ast_program = match ast::Program::new(&mut lexxed_file)
    {
        Some(it) => it,
        _ => return,
    };

    let asm_string = generate_asm(&ast_program);
    let last_dot_pos = args[1].rfind('.').unwrap();
    let out_file = &args[1][..last_dot_pos];
    let mut out_file = File::create(out_file).unwrap();
    out_file.write_all(asm_string.as_bytes()).unwrap();
}


