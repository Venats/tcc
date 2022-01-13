use crate::lexxer::*;
use std::collections::VecDeque;


fn validate_rules(req_tokens : &Vec<LexToken>, tokens : &mut VecDeque<LexToken>) -> bool
{
    if tokens.len() < req_tokens.len()
    {
        println!("FAILURE, Not enough tokens left");
        return false;
    }

    for (match_token,req_token)  in tokens.drain(0..req_tokens.len()).zip(req_tokens)
    {
        if *req_token != match_token
        {
            println!("FAILURE, Expected Token {:?}, Found token {:?}", req_token,match_token);
            return false;
        }
    }
    true
}

fn validate_rule(req_token : LexToken, tokens : &mut VecDeque<LexToken>) -> bool
{
    if let Some(token) = tokens.pop_front()
    {
        if req_token == token 
        {
            return true;
        }
        else
        {
            println!("FAILURE, Expected Token {:?}, Found token {:?}", req_token,token);
        }
        return req_token == token
    }
    false

}


#[derive(Debug)]
pub enum Constant
{
    Integer(String),
}

#[derive(Debug)]
pub enum Expression
{
    Constant(Constant),
}

#[derive(Debug)]
pub enum Statement
{
    Return(Expression),
}

#[derive(Debug)]
pub struct FunctionDecl
{
    name : String,
    body : Statement,
}

#[derive(Debug)]
pub enum Program
{
    Declaration(FunctionDecl),
}

impl Constant 
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Constant>
    {
        if let Some(LexToken::IntLiteral(int_str)) = tokens.pop_front()
        {
            if let Ok(_) = int_str.parse::<i32>()
            {
                return Some(Constant::Integer(int_str));
            }
        }
        println!("Error parsing the Constant");
        None
    }
}


impl Expression 
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Expression>
    {
        if let Some(cons) = Constant::new(tokens)
        {
            return Some(Expression::Constant(cons));
        }
        println!("Error parsing the Expression");
        None
    }

}


impl Statement
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Statement>
    {
        if validate_rule(LexToken::Return, tokens)
        {
            if let Some(expression) = Expression::new(tokens)
            {
                if validate_rule(LexToken::Semicolon, tokens)
                {
                    return Some(Statement::Return(expression));
                }
            }
        }
        println!("Error parsing the Statement");
        None
    }
}


impl FunctionDecl 
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<FunctionDecl>
    {
        if validate_rule(LexToken::Int, tokens )
        {
            if let Some(LexToken::Identifier(func_name)) = tokens.pop_front()
            {
                let func_lex_tokens = vec!{LexToken::OpenParenth
                            ,LexToken::CloseParenth
                            ,LexToken::OpenBrace};
                if validate_rules(&func_lex_tokens, tokens)
                {
                    if let Some(func_body) = Statement::new(tokens)
                    {
                        return Some(FunctionDecl{name: func_name, body: func_body});
                    }
                }
            }
            else {
                println!("FAILED WITH ID");
            }
        }
        println!("Error parsing the Function Decl");
        None
    }
}

impl Program 
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Program>
    {
        if let Some(decl) = FunctionDecl::new(tokens)
        {
            return Some(Program::Declaration(decl));
        } 
        println!("Error parsing the Program");
        None
    }
}