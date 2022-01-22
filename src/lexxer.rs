use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


#[derive(Debug,PartialEq,Eq,Clone)]
pub enum LexToken
{
    OpenBrace,
    CloseBrace,
    OpenParenth,
    CloseParenth,
    Semicolon,
    Int,
    Return,
    Identifier(String),
    IntLiteral(String),
    Undefined
}

impl LexToken
{
    fn from_str(lex_str : &str) -> LexToken
    {
        match lex_str {
            "{" => return LexToken::OpenBrace,
            "}" => return LexToken::CloseBrace,
            "(" => return LexToken::OpenParenth,
            ")" => return LexToken::CloseParenth,
            ";" => return LexToken::Semicolon,
            "int" => return LexToken::Int,
            "return" => return LexToken::Return,
            _ =>
            {
                if let Some(id) = Regex::new(r"[a-zA-Z]\w*").unwrap().captures(lex_str) 
                {
                    return LexToken::Identifier(id.get(0).unwrap().as_str().to_string());
                }
                else if let Some(int_literal) = Regex::new(r"[0-9]+").unwrap().captures(lex_str) 
                {
                    return LexToken::IntLiteral(int_literal.get(0).unwrap().as_str().to_string());
                }
            }
        }
        return LexToken::Undefined;
    }
    fn to_str(&self) -> Option<&str> {
        match self {
            LexToken::OpenBrace => return Some("{"),
            LexToken::CloseBrace => return Some("}"),
            LexToken::OpenParenth => return Some("("),
            LexToken::CloseParenth => return Some(")"),
            LexToken::Semicolon => return Some(";"),
            LexToken::Int => return Some("int"),
            LexToken::Return => return Some("return"),
            LexToken::Identifier(id) => return Some(id),
            LexToken::IntLiteral(int) => return Some(int),
            LexToken::Undefined => return None
        }
    }
}


pub fn lex(file_path : &str) -> VecDeque<LexToken> 
{
    // let source_str = fs::read_to_string(file_name).unwrap();
    // source_str.replace("\n",)
    let mut lex_vec = VecDeque::new();
    if let Ok(lines) = read_lines(file_path)
    {
        for line in lines
        {
            if let Ok(l) = line 
            {
                let re = Regex::new(r"[\{\}\(\);]").unwrap();
                let space_l = re.replace_all(&l," $0 ").into_owned();
                let mut lex_line : VecDeque<LexToken> = space_l.split_whitespace().map(|word| LexToken::from_str(word)).collect();
                lex_vec.append(&mut lex_line);
            }
        }
    }
    return lex_vec
}


#[test]
fn test_from_str() 
{
    assert_eq!(LexToken::OpenBrace, LexToken::from_str("{"));
    assert_eq!(LexToken::CloseBrace, LexToken::from_str("}"));
    assert_eq!(LexToken::OpenParenth, LexToken::from_str("("));
    assert_eq!(LexToken::CloseParenth, LexToken::from_str(")"));
    assert_eq!(LexToken::Semicolon, LexToken::from_str(";"));
    assert_eq!(LexToken::Int, LexToken::from_str("int"));
    assert_eq!(LexToken::Return, LexToken::from_str("return"));
    assert_eq!(LexToken::Identifier("main".to_string()), LexToken::from_str("main"));
    assert_eq!(LexToken::IntLiteral("99".to_string()), LexToken::from_str("99"));
    assert_eq!(LexToken::Undefined, LexToken::from_str("..."));
}

#[test]
fn test_to_str() 
{
    assert_eq!(LexToken::OpenBrace.to_str(), Some("{"));
    assert_eq!(LexToken::CloseBrace.to_str(), Some("}"));
    assert_eq!(LexToken::OpenParenth.to_str(), Some("("));
    assert_eq!(LexToken::CloseParenth.to_str(), Some(")"));
    assert_eq!(LexToken::Semicolon.to_str(), Some(";"));
    assert_eq!(LexToken::Int.to_str(), Some("int"));
    assert_eq!(LexToken::Return.to_str(), Some("return"));
    assert_eq!(LexToken::Identifier("main".to_string()).to_str(), Some("main"));
    assert_eq!(LexToken::IntLiteral("99".to_string()).to_str(), Some("99"));

    assert_eq!(LexToken::Undefined.to_str(),None);
}
