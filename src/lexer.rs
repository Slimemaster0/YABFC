pub enum Token {
    PlusToken,
    MinusToken,
    RightToken,
    LeftToken,
    SqBraceR,
    SqBraceL,
    InputToken,
    OutputToken,
    VoidToken(char)
}

fn tokenize(input: char) -> Token {
    match input {
        '+' => return Token::PlusToken,
        '-' => return Token::MinusToken,
        '>' => return Token::RightToken,
        '<' => return Token::LeftToken,
        '[' => return Token::SqBraceR,
        ']' => return Token::SqBraceL,
        ',' => return Token::InputToken,
        '.' => return Token::OutputToken,
        _ => return Token::VoidToken(input.to_owned()),
    }
}

pub fn lexer(input: &String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect::<Vec<_>>();
    let mut tokens: Vec<Token> = Vec::new();
    
    for i in 0..chars.len() {
        tokens.push(tokenize(chars[i]));
    }
    return tokens;
}
