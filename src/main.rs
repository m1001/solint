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

enum PossibleToken {
    GoodToken(Token),
    NoToken,
}


// what we need to do
// pass all the chars to find a token one after another
//  on each char --
//      check if the PossibleToken value is NoToken or a GoodToken
//      if it's a good token, we need a new char list without the previous chars
//      problem: whitespace!

fn get_tokens(fp_s: &str) -> (PossibleToken, &str) {
    let token = match fp_s {
        "\n" => Token::Newline("linefeed".to_string()),
        " " => {
                let re = Regex::new("[a-zA-Z0-9] [a-zA-Z0-9]");
                if re.is_match(fp_s) {
                    Token::Whitespace("single_whitespace".to_string())
                } else {
                    Token::Whitespace("multi_whitespace".to_string())
                }
        }
        "    " | 
        "        " |
        "            " => Token::Indent(fp_s.to_string()), 
        "\t" => Token::Indent("tab_indent".to_string()),
        "contract" |
        "struct" |
        "bytes32" |
        "uint" | 
        "address" |
        "bool" => Token::Keyword(fp_s.to_string()),
        "!" | 
        " && " |
        "//" |
        "==" |
        "<=" | 
        "<" |
        ">" | 
        ">=" |
        "&" |
        "/" |
        "^" |
        "~" |
        "+" |
        "-" |
        "*" |
        "%" |
        "**" |
        "!=" => Token::Operator(fp_s.to_string()),
        _ => Token::Unknown(fp_s.to_string()),
    };
    let possible_token = match token {
        Token::Unknown(ref s) => PossibleToken::NoToken,
        _ => PossibleToken::GoodToken(token),
    };
    (possible_token, fp_s)
}


fn find_token(chunk: &str) -> Token {
    let ret = match chunk {
        "\n" => Token::Newline("linefeed".to_string()),
        " " => Token::Whitespace("whitespace".to_string()),
        "    " | 
        "        " |
        "            " => Token::Indent(chunk.to_string()), 
        "\t" => Token::Indent("tab_indent".to_string()),
        "contract" |
        "struct" |
        "bytes32" |
        "uint" | 
        "address" |
        "bool" => Token::Keyword(chunk.to_string()),
        "!" | 
        " && " |
        "//" |
        "==" |
        "<=" | 
        "<" |
        ">" | 
        ">=" |
        "&" |
        "/" |
        "^" |
        "~" |
        "+" |
        "-" |
        "*" |
        "%" |
        "**" |
        "!=" => Token::Operator(chunk.to_string()),
        _ => Token::Unknown(chunk.to_string()),
        //_ => Token::Unknown("**unknown**".to_string()),
    };
    ret
}

fn parse_tokens(file_text: &str) -> Vec<Token> {
    let chars: Vec<char> = file_text.chars().collect();
    let mut prev_chunk = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    for c in chars {
        prev_chunk.push(c);
        let token = find_token(&prev_chunk);
        match token {
            Token::Unknown(ref s) => { 
                if prev_chunk.len() >= 10 {
                    prev_chunk = String::new()
                }
            },
            _ => {
                prev_chunk = prev_chunk;
            },
        }
        tokens.push(token); 
    }
    for token in &tokens {
        println!("{}", token.get_string());
    }
    tokens
} 

fn main() {
    get_args();
    let token = Token::Newline("\n".to_string());
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
        tokens = parse_tokens(&fp_s);

    } else {
        println!("{}:  file not found", &input_file);
        help_menu();
        return;
    }
    println!("got {} tokens from input file", tokens.len());
}   
