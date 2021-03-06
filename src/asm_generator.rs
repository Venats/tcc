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
    return format!(".globl {}\n{}:\n{}", &func_decl.name,&func_decl.name, statement_asm(&func_decl.body));
}

fn statement_asm(statement : &Statement) -> String
{
    match statement
    {
        Statement::Return(exp) => return format!("{}\nret\n", expression_asm(&exp)),
    };
}

fn expression_asm(expression : &Expression) -> String
{
    match expression
    {
        // Expression::Constant(const_str) => return format!("{}", constant_asm(&const_str)),
        // Expression::UnOp(oper, exp) => return format!("{}\n{}",expression_asm(exp), unioperator_asm(oper)),
        Expression::BinOp(oper, exp,expr) => return "".to_string(),
        _ => return "".to_string(), //TODO:REMOVE
    };
}

fn unioperator_asm(oper : &UniOperator) -> String
{
    match oper
    {
        UniOperator::Negation => return String::from("neg %eax\n"),
        UniOperator::BitwiseComplement => String::from("not %eax\n"),
        UniOperator::LogicalNegation => return String::from("cmpl $0, %eax\nmovl $0, %eax\nsete %al\n"),
    }
}

fn constant_asm(constant : &Constant) -> String
{
    match constant
    {
        Constant::Integer(string) => return format!("movl ${}, %eax",string),
    };
}
