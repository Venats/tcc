use crate::ast::*;


pub fn generate_asm(program_ast : &Program) -> String
{
    match program_ast
    {
        Program::Declaration(func_decl) => return function_asm(&func_decl),
    };
}

fn function_asm(func_decl : &FunctionDecl) -> String
{
    return format!(".global _{}\n{}:\n{}", &func_decl.name,&func_decl.name, statement_asm(&func_decl.body));
}

fn statement_asm(statement : &Statement) -> String
{
    match statement
    {
        Statement::Return(exp) => return format!("movl ${}, %eax\nret", expression_asm(&exp)),
    };
}

fn expression_asm(expression : &Expression) -> String
{
    match expression
    {
        Expression::Constant(const_str) => return format!("{}", constant_asm(&const_str)),
    };
}

fn constant_asm(constant : &Constant) -> String
{
    match constant
    {
        Constant::Integer(string) => return string.clone(),
    };
}
