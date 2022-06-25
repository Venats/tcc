use crate::lexxer::*;
use std::{collections::VecDeque, f32::consts::E};


#[derive(Debug)]
pub enum Constant
{
    Integer(String),
}

#[derive(Debug)]
pub enum UniOperator
{
    Negation,
    BitwiseComplement,
    LogicalNegation,
}

#[derive(Debug)]
pub enum BiOperator
{
    Addition,
    Subtraction,
    Multiplication,
    Division,
}


#[derive(Debug)]
pub enum Expression
{
    Term(Box<Term>, Vec<(BiOperator, Term)>),
}


#[derive(Debug)]
pub enum Term
{
    Factor(Factor, Vec<(BiOperator, Factor)>)
}

#[derive(Debug)]
pub enum Factor
{
    Constant(Constant),
    UnOp(UniOperator, Box<Factor>),
    Expr(Expression)
}

#[derive(Debug)]
pub enum Statement
{
    Return(Expression),
}

#[derive(Debug)]
pub struct FunctionDecl
{
    pub name : String,
    pub body : Statement,
}

#[derive(Debug)]
pub enum Program
{
    Declaration(FunctionDecl),
}

impl Constant 
{
    pub fn new(token : &LexToken) -> Option<Constant>
    {
        match token
        {
            LexToken::IntLiteral(int_str) => {
                if let Ok(_) = int_str.parse::<i32>()
                {
                    return Some(Constant::Integer(int_str.to_owned()));
                }
                else
                {
                    println!("Error parsing the Constant");
                    return None;
                }
            },
            _ => return None,
        };
    }
}

impl UniOperator
{
    pub fn new(token: &LexToken) -> Option<UniOperator>
    {
        match token
        {
            LexToken::Negation => return Some(UniOperator::Negation),
            LexToken::BitwiseComplement => return Some(UniOperator::BitwiseComplement),
            LexToken::LogicalNegation => return Some(UniOperator::LogicalNegation),
            _ => return None,
        };
    }
}

impl BiOperator
{
    pub fn new(token: &LexToken) -> Option<BiOperator>
    {
        match token
        {
            LexToken::Addition => return Some(BiOperator::Addition),
            LexToken::Negation => return Some(BiOperator::Subtraction),
            LexToken::Multiplication => return Some(BiOperator::Multiplication),
            LexToken::Division => return Some(BiOperator::Division),
            _ => return None,
        };
    }
}


impl Expression 
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Expression>
    {
        if let Some(term) = Term::new(tokens)
        {
            let mut next_terms = Vec::new();
            
            while let Some(token) = tokens.pop_front()
            {
                if token == LexToken::Addition ||
                    token == LexToken::Negation
                {
                    if let Some(next_term) = Term::new(tokens)
                    {
                        next_terms.push((BiOperator::new(&token).unwrap(),next_term));
                    }
                    else
                    {
                        return None;
                    }
                }
                else
                {
                    tokens.push_front(token);
                    break;
                }
            }
            return Some(Expression::Term(Box::new(term),next_terms));
        }
        println!("Failed generating expression");
        return None;
    }
}


impl Term
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Term>
    {
        if let Some(factor) = Factor::new(tokens)
        {
            let mut next_factors = Vec::new();
            
            while let Some(token) = tokens.pop_front()
            {
                if token == LexToken::Multiplication ||
                    token == LexToken::Division
                {
                    if let Some(next_factor) = Factor::new(tokens)
                    {
                        next_factors.push((BiOperator::new(&token).unwrap(),next_factor));
                    }
                    else
                    {
                        return None;
                    }
                }
                else
                {
                    tokens.push_front(token);
                    break;
                }
            }
            return Some(Term::Factor(factor,next_factors));
        }
        return None;
    }
}

impl Factor
{
    pub fn new(tokens : &mut VecDeque<LexToken>) -> Option<Factor>
    {
        if let Some(token) = tokens.pop_front() 
        {
            match token
            {
                LexToken::OpenParenth => {
                    let maybe_expr = Expression::new(tokens);
                    let maybe_close = tokens.pop_front();

                    if maybe_expr.is_some() &&
                        maybe_close == Some(LexToken::CloseParenth)
                    {
                        return Some(Factor::Expr(maybe_expr.unwrap()));
                    }
                    return None;
                }
                LexToken::IntLiteral(_) => { 
                    if let Some(cons) = Constant::new(&token)
                    {
                        return Some(Factor::Constant(cons));
                    }
                    println!("Failed generating factor");
                    return None;
                },
                _ => {
                    if let Some(oper) = UniOperator::new(&token)
                    {
                        if let Some(factor) = Factor::new(tokens)
                        {
                            return Some(Factor::UnOp(oper,Box::new(factor)));
                        }
                    }
                    println!("Failed generating factor");
                    return None;
                },
            }
        }
        return None;
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
                        if validate_rule(LexToken::CloseBrace, tokens)
                        {
                            return Some(FunctionDecl{name: func_name, body: func_body});
                        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn make_deq(tokens : Vec<LexToken>) -> VecDeque<LexToken>{
        tokens.into_iter().collect()
    }

    #[test]
    fn validate_rule_true()
    {
        let mut deq = make_deq(vec!(LexToken::Semicolon));
        assert_eq!(true, validate_rule(LexToken::Semicolon, &mut deq));
        assert_eq!(0,deq.len());
    }

    #[test]
    fn validate_rule_false()
    {
        let mut deq = make_deq(vec!(LexToken::CloseBrace));
        assert_eq!(false, validate_rule(LexToken::Semicolon, &mut deq));
        assert_eq!(0,deq.len());
    }

    #[test]
    fn validate_rules_true()
    {
        let mut deq = make_deq(vec!(LexToken::Int
                        , LexToken::Identifier(String::from("main"))
                        , LexToken::OpenParenth
                        , LexToken::CloseParenth));
        let test = vec!(LexToken::Int
        , LexToken::Identifier(String::from("main"))
        , LexToken::OpenParenth
        , LexToken::CloseParenth);
        assert_eq!(true, validate_rules(&test, &mut deq));
        assert_eq!(0,deq.len());
    }

    #[test]
    fn validate_rules_false()
    {
        let mut deq = make_deq(vec!(LexToken::Int
            , LexToken::OpenParenth
            , LexToken::Identifier(String::from("main"))
            , LexToken::CloseParenth));
        let test = vec!(LexToken::Int
        , LexToken::Identifier(String::from("main"))
        , LexToken::OpenParenth
        , LexToken::CloseParenth);

        assert_eq!(false, validate_rules(&test, &mut deq));
        assert_eq!(0,deq.len());
    }
}