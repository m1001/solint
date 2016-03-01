extern crate getopts;
extern crate regex;
use getopts::Options;
use std::env;
use std::fs::File;
use std::path::Path;
use std::io::Read;
use regex::Regex;

#[derive(Debug)]
enum Token {
    Newline(String),
    Indent(String),
    Identifier(String),
    Keyword(String),
    Operator(String),
    Whitespace(String),
    Unknown(String),
}

impl Token {
    fn get_string(&self) -> String {
        match self {
            &Token::Unknown(ref s) |
            &Token::Newline(ref s) |
            &Token::Indent(ref s) |
            &Token::Keyword(ref s) |
            &Token::Operator(ref s) |
            &Token::Whitespace(ref s) |
            &Token::Identifier(ref s) => s.clone(),
        }
    }
}

fn match_token(ts: &str) -> Token {
    match ts {
        "\n" => Token::Newline(ts.to_string()),
        " " => Token::Whitespace(ts.to_string()),
        "address" |
        "bool" |
        "bytes32" |
        "contract" |
        "else" |
        "event" |
        "for" |
        "function" |
        "if" |
        "int" |
        "mapping" |
        "modifier" |
        "public" |
        "real" |
        "struct" |
        "this" |
        "throw" |
        "uint" |
        "var" |
        "while" => Token::Keyword(ts.to_string()),
        "!" |
        "(" |
        "" |
        "//" |
        "=" |
        ">=" |
        "<" |
        ">" |
        "&" |
        "/" |
        "^" |
        "~" |
        "+" |
        "{" |
        "}" |
        ";" |
        ":" |
        "-" |
        "*" |
        "%" => Token::Identifier(ts.to_string()),
        _ => Token::Unknown(ts.to_string()),
    }

}


fn ninja_tokens(fp: &str) -> Vec<Token> {
    let mut prev_chunk = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    for c in fp.chars() {
        prev_chunk.push(c);
        let s_token = match_token(&prev_chunk);
        let c_token = match_token(&c.to_string());
        let s_match = &prev_chunk == &c.to_string();
        match s_token {
            Token::Unknown(ref s) => prev_chunk = prev_chunk,
            _ => {
                tokens.push(s_token);
                prev_chunk = String::new();
            }
        }
        match c_token {
            Token::Unknown(ref s) => prev_chunk = prev_chunk,
            _ => {
                if !s_match {
                    tokens.push(c_token);
                    prev_chunk = String::new();
                }
            }
        }
   }
    tokens
}

fn main() {
    get_args();
}

fn help_menu() {
    let out = "Usage: solint -i FILE [options]";
    println!("{}", out);
}

fn get_args() {
    let user_args: Vec<String> = env::args().collect();
    let this_app = user_args[0].clone();
    let mut available_opts = Options::new();
    available_opts.optopt("i", "", "set input file(s)", "INPUT");
    available_opts.optflag("h", "help", "print this menu");
    let matches = match available_opts.parse(&user_args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    parse_args(matches);
}

fn parse_args(matches: getopts::Matches) {
    let mut tokens: Vec<Token> = Vec::new();
    let input_file = match matches.opt_str("i") {
        Some(x) => x,
        _ => "**undefined**".to_string(),
    };
 
    if matches.opt_present("h") ||  input_file == "**undefined**".to_string() {
        help_menu();
        return;
    }
    if Path::exists(Path::new(&input_file)) {
        println!("parsing {}", &input_file);
        let mut fp = File::open(&input_file).unwrap();
        let mut fp_s = String::new();
        fp.read_to_string(&mut fp_s).unwrap();
        tokens = ninja_tokens(&fp_s);

    } else {
        println!("{}:  file not found", &input_file);
        help_menu();
        return;
    }
    println!("got {} tokens from input file", tokens.len());
    for t in tokens { println!("{:?}", t); }
}   
