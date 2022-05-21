
use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

const TOKENS_NEED_SPACE_REGEX : &str = r"[/+\*\{\}\(\);!~-]";


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn format_spacing(string : &str) -> String
{
    let re = Regex::new(TOKENS_NEED_SPACE_REGEX).unwrap();
    return re.replace_all(&string," $0 ").into_owned();
}

fn from_line(line : &str) -> VecDeque<LexToken>
{
    return format_spacing(line).split_whitespace().map(|word| LexToken::from_str(word)).collect();
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
    Negation,
    BitwiseComplement,
    LogicalNegation,
    Addition,
    Multiplication,
    Division,
    Undefined
}

impl LexToken
{
    fn from_str(lex_str : &str) -> LexToken
    {
        match lex_str {
            "{"         => return LexToken::OpenBrace,
            "}"         => return LexToken::CloseBrace,
            "("         => return LexToken::OpenParenth,
            ")"         => return LexToken::CloseParenth,
            ";"         => return LexToken::Semicolon,
            "int"       => return LexToken::Int,
            "return"    => return LexToken::Return,
            "-"         => return LexToken::Negation,
            "~"         => return LexToken::BitwiseComplement,
            "!"         => return LexToken::LogicalNegation,
            "+"         => return LexToken::Addition,
            "*"         => return LexToken::Multiplication,
            "/"         => return LexToken::Division,
            _           =>
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
            LexToken::OpenBrace                 => return Some("{"),
            LexToken::CloseBrace                => return Some("}"),
            LexToken::OpenParenth               => return Some("("),
            LexToken::CloseParenth              => return Some(")"),
            LexToken::Semicolon                 => return Some(";"),
            LexToken::Int                       => return Some("int"),
            LexToken::Return                    => return Some("return"),
            LexToken::Negation                  => return Some("-"),
            LexToken::BitwiseComplement         => return Some("~"),
            LexToken::LogicalNegation           => return Some("!"),
            LexToken::Identifier(id)    => return Some(id),
            LexToken::IntLiteral(int)   => return Some(int),
            LexToken::Addition                  => return Some("+"),
            LexToken::Multiplication            => return Some("*"),
            LexToken::Division                  => return Some("/"),
            LexToken::Undefined => return None
        }
    }
}



pub fn lex(file_path : &str) -> VecDeque<LexToken> 
{
    let mut lex_vec = VecDeque::new();
    if let Ok(lines) = read_lines(file_path)
    {
        for line in lines
        {
            if let Ok(l) = line
            {
                lex_vec.append(&mut from_line(&l));
            }
        }
    }
    return lex_vec
}


#[cfg(test)]
mod test
{
    use super::*;

    fn generate_stage_2_vec(un_oper : LexToken, int_literal: &str) -> VecDeque<LexToken>
    {
        let mut stage2_vec = VecDeque::new();
        stage2_vec.push_back(LexToken::Int);
        stage2_vec.push_back(LexToken::Identifier(String::from("main")));
        stage2_vec.push_back(LexToken::OpenParenth);
        stage2_vec.push_back(LexToken::CloseParenth);
        stage2_vec.push_back(LexToken::OpenBrace);
        stage2_vec.push_back(LexToken::Return);
        stage2_vec.push_back(un_oper);
        stage2_vec.push_back(LexToken::IntLiteral(String::from(int_literal.to_string())));
        stage2_vec.push_back(LexToken::Semicolon);
        stage2_vec.push_back(LexToken::CloseBrace);
        return stage2_vec;
    }

    fn generate_stage_3_vec(oper : LexToken, int_literal: &str) -> VecDeque<LexToken>
    {
        let mut stage3_vec = VecDeque::new();
        stage3_vec.push_back(LexToken::Int);
        stage3_vec.push_back(LexToken::Identifier(String::from("main")));
        stage3_vec.push_back(LexToken::OpenParenth);
        stage3_vec.push_back(LexToken::CloseParenth);
        stage3_vec.push_back(LexToken::OpenBrace);
        stage3_vec.push_back(LexToken::Return);
        stage3_vec.push_back(LexToken::IntLiteral(String::from(int_literal.to_string())));
        stage3_vec.push_back(oper);
        stage3_vec.push_back(LexToken::IntLiteral(String::from(int_literal.to_string())));
        stage3_vec.push_back(LexToken::Semicolon);
        stage3_vec.push_back(LexToken::CloseBrace);
        return stage3_vec;
    }

    #[test]
    fn test_simple_format_spacing()
    {
        assert_eq!(format_spacing("hello;"), "hello ; ");
        assert_eq!(format_spacing("hello}"), "hello } ");
        assert_eq!(format_spacing("hello{"), "hello { ");
        assert_eq!(format_spacing("hello)"), "hello ) ");
        assert_eq!(format_spacing("hello("), "hello ( ");
        assert_eq!(format_spacing("!hello"), " ! hello");
        assert_eq!(format_spacing("~hello"), " ~ hello");
        assert_eq!(format_spacing("-hello"), " - hello");
    }
    
    #[test]
    fn test_format_spacing()
    {
        assert_eq!(format_spacing("-!~{hello"), " -  !  ~  { hello");
        assert_eq!(format_spacing("void main(){return 3;}"), "void main (  )  { return 3 ;  } ");
        assert_eq!(format_spacing("int main(int argc) {return ~1; }"),
                     "int main ( int argc )   { return  ~ 1 ;   } ");
    }
    
    
    #[test]
    fn test_stage1_from_line()
    {
        let mut stage1_vec = VecDeque::new();
        stage1_vec.push_back(LexToken::Int);
        stage1_vec.push_back(LexToken::Identifier(String::from("main")));
        stage1_vec.push_back(LexToken::OpenParenth);
        stage1_vec.push_back(LexToken::CloseParenth);
        stage1_vec.push_back(LexToken::OpenBrace);
        stage1_vec.push_back(LexToken::Return);
        stage1_vec.push_back(LexToken::IntLiteral(String::from("3")));
        stage1_vec.push_back(LexToken::Semicolon);
        stage1_vec.push_back(LexToken::CloseBrace);
        assert_eq!(from_line("int main(){return 3;}"), stage1_vec)
    }

    #[test]
    fn test_stage2_from_line()
    {
        assert_eq!(from_line("int main(){return !3;}"), generate_stage_2_vec(LexToken::LogicalNegation, "3"));
        assert_eq!(from_line("int main(){return -3;}"), generate_stage_2_vec(LexToken::Negation, "3"));
        assert_eq!(from_line("int main(){return ~3;}"), generate_stage_2_vec(LexToken::BitwiseComplement, "3"));
    }

    #[test]
    fn test_stage3_from_line()
    {
        assert_eq!(from_line("int main(){return 2+2;}"), generate_stage_3_vec(LexToken::Addition,"2"));
        assert_eq!(from_line("int main(){return 2 + 2;}"), generate_stage_3_vec(LexToken::Addition,"2"));
        assert_eq!(from_line("int main(){return 2*2;}"), generate_stage_3_vec(LexToken::Multiplication,"2"));
        assert_eq!(from_line("int main(){return 2 * 2;}"), generate_stage_3_vec(LexToken::Multiplication,"2"));
        assert_eq!(from_line("int main(){return 2/2;}"), generate_stage_3_vec(LexToken::Division,"2"));
        assert_eq!(from_line("int main(){return 2 / 2;}"), generate_stage_3_vec(LexToken::Division,"2"));
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
        assert_eq!(LexToken::Negation, LexToken::from_str("-"));
        assert_eq!(LexToken::BitwiseComplement, LexToken::from_str("~"));
        assert_eq!(LexToken::LogicalNegation, LexToken::from_str("!"));
        assert_eq!(LexToken::Identifier(String::from("main")), LexToken::from_str("main"));
        assert_eq!(LexToken::IntLiteral(String::from("99")), LexToken::from_str("99"));
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
        assert_eq!(LexToken::Negation.to_str(), Some("-"));
        assert_eq!(LexToken::BitwiseComplement.to_str(), Some("~"));
        assert_eq!(LexToken::LogicalNegation.to_str(), Some("!"));
        assert_eq!(LexToken::Identifier(String::from("main")).to_str(), Some("main"));
        assert_eq!(LexToken::IntLiteral(String::from("99")).to_str(), Some("99"));
    
        assert_eq!(LexToken::Undefined.to_str(),None);
    }
    
}