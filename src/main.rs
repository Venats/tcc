use std::env;

mod ast;
mod lexxer;
use crate::lexxer::*;

// static assembly_format: &str = "
//         .globl _main
//         _main:
//             movl    ${}, %eax
//             ret
// ";



fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lexxed_file = lexxer::lex(&args[1]);
    println!("{:?}", lexxed_file);

    if let Some(ast) = ast::Program::new(&mut lexxed_file)
    {
        println!("{:?}",ast);
    }
//     let assembly_file = str::replace(&args[1], ".c", ".s");
//     let source_str = fs::read_to_string(&args[1]).unwrap();
    
//     let source_match = source_re.captures(&source_str).unwrap();
//     let retval = source_match.name("ret").unwrap().as_str();
//     fs::write(assembly_file,format!("
//     .globl _main
//     _main:
//         movl    ${}, %eax
//         ret
// ",retval)).unwrap();
    println!("Hello, world!");
}
