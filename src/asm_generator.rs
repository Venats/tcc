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
        Expression::Constant(const_str) => return format!("{}", constant_asm(&const_str)),
        Expression::UnOp(oper, exp) => return format!("{}\n{}",expression_asm(exp), operator_asm(oper)),
    };
}

fn operator_asm(oper : &Operator) -> String
{
    match oper
    {
        Operator::Negation => return String::from("neg %eax\n"),
        Operator::BitwiseComplement => String::from("not %eax\n"),
        Operator::LogicalNegation => return String::from("cmpl $0, %eax\nmovl $0, %eax\nsete %al\n"),
    }
}

fn constant_asm(constant : &Constant) -> String
{
    match constant
    {
        Constant::Integer(string) => return format!("movl ${}, %eax",string),
    };
}
