use std::env;
use std::fs;
use std::io::prelude::*;
use std::process::Command;

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
    let mut out_file_path = args[1][..last_dot_pos].to_string();
    out_file_path += ".s";
    let mut out_file = fs::File::create(&out_file_path).unwrap();
    out_file.write_all(asm_string.as_bytes()).unwrap();

    Command::new("gcc")
                    .arg("-m32")
                    .arg(&out_file_path)
                    .arg("-o")
                    .arg(&args[1][..last_dot_pos])
                    .output()
                    .expect("Failed to execute gcc command");
}


