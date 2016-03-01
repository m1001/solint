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

fn scan_char(c: char) -> bool {
    match c {
        '\n'|
        '\t'|
        ' ' |
        'a' |
        'b' |
        'c' |
        'e' |
        'f' |
        'i' |
        'm' |
        'p' |
        'r' |
        's' |
        't' |
        'u' |
        '!' | 
        'v' |
        '(' |
        ')' |
        '=' |
        '<' | 
        '>' | 
        '&' |
        '/' |
        '^' |
        '~' |
        '+' |
        '{' |
        '}' |
        ';' |
        ':' |
        '-' |
        '%' |
        '*' => true,
        _  => false
    }
}

fn get_valid_keywords() -> Vec<String> {
    vec!(
        "\n".to_string(),
        " ".to_string(),
        "    ".to_string(),
        "\t".to_string(),
        "address".to_string(),
        "bool".to_string(),
        "bytes32".to_string(),
        "contract".to_string(),
        "else".to_string(),
        "event".to_string(),
        "for".to_string(),
        "function".to_string(),
        "if".to_string(),
        "int".to_string(),
        "mapping".to_string(),
        "modifier".to_string(),
        "public".to_string(),
        "real".to_string(),
        "struct".to_string(),
        "this".to_string(),
        "throw".to_string(),
        "uint".to_string(),
        "var".to_string(),
        "while".to_string(),
        "!".to_string(),
        "(".to_string(),
        ")".to_string(),
        "//".to_string(),
        "=".to_string(),
        "!=".to_string(),
        "==".to_string(),
        "<=".to_string(),
        ">=".to_string(),
        "<".to_string(),
        ">".to_string(),
        "&".to_string(),
        "/".to_string(),
        "^".to_string(),
        "~".to_string(),
        "+".to_string(),
        "{".to_string(),
        "}".to_string(),
        ";".to_string(),
        ":".to_string(),
        "-".to_string(),
        "*".to_string(),
        "**".to_string(),
        "%".to_string(),
    )

}

fn fetch_tokens(fp: &str) -> Vec<Token> {
    let chars: Vec<char> = fp.chars().collect();
    let mut prev_chunk = String::new();
    let mut tokens: Vec<Token> = Vec::new();
    let mut scanning = false;
    let mut was_scanning = false;
    let keywords = get_valid_keywords();
    for c in chars {
        prev_chunk.push(c);
        scanning = scan_char(c);
        if scanning == true {
            if was_scanning == true {
                if keywords.contains(&prev_chunk) {
                    println!("tokenzz: -{}-",prev_chunk);
                    was_scanning = false;
                    prev_chunk = String::new();
                } else if keywords.contains(&c.to_string()) {
                    println!("tokenccc: -{}-", c);
                    prev_chunk = String::new();
                }
            } else {
                if keywords.contains(&c.to_string()) {
                    println!("tokencc: -{}-", c);
                    prev_chunk = String::new();
                }
            }
            if keywords.contains(&prev_chunk) {
                println!("token: -{}-",&prev_chunk);
                prev_chunk = String::new();
            }
            was_scanning = true;
        } else { 
            if keywords.contains(&prev_chunk) {
                println!("tokenxx: {}", prev_chunk);
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
        tokens = fetch_tokens(&fp_s);

    } else {
        println!("{}:  file not found", &input_file);
        help_menu();
        return;
    }
    println!("got {} tokens from input file", tokens.len());
}   
