
#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Input,
    Output,
    Star,
    SetTo,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    OpeningParen,
    ClosingParen,
    Identifier(String),
    Str(String),
    Num(i32),
    Operator(Operator),
    True,
    False,
    FileEnd,
}
pub struct Lexer {
    pub buf: Vec<char>,
    pub ptr: usize,
    pub last: Token,
}
impl Lexer {
    pub fn getch(&mut self) -> char {
        let ch: char = self.buf[self.ptr];
        // let ch: char = self.buf.as_bytes()[self.ptr] as char;
        self.ptr += 1;
        return ch;
    }
    pub fn backch(&mut self){
        self.ptr -= 1;
    }
    pub fn next_token(&mut self) -> Token {
        if self.ptr >= self.buf.len() {
            return Token::FileEnd;
        }
        let mut ch: char = self.getch();
        self.last = if ch.is_alphanumeric() {
            let mut tempbuf: String = String::new();
            while ch.is_alphanumeric() {
                tempbuf.push(ch);
                ch = self.getch();
            }
            self.backch();
            match tempbuf.parse::<i32>() {
                Ok(n) => Token::Num(n),
                Err(_) => match tempbuf.as_str() {
                    "true" => Token::True,
                    "false" => Token::False,
                    _ => Token::Identifier(tempbuf),
                },
            }
        }
        else {
            match ch {
                '(' => Token::OpeningParen,
                ')' => Token::ClosingParen,
                '<' => Token::Operator(Operator::Input),
                '>' => Token::Operator(Operator::Output),
                ':' => Token::Operator(Operator::Star),
                '=' => Token::Operator(Operator::SetTo),
                '+' => Token::Operator(Operator::Add),
                '-' => Token::Operator(Operator::Sub),
                '*' => Token::Operator(Operator::Mul),
                '/' => Token::Operator(Operator::Div),
                '%' => Token::Operator(Operator::Mod),
                '&' => Token::Operator(Operator::And),
                '|' => Token::Operator(Operator::Or),
                '!' => Token::Operator(Operator::Not),
                '"' => {
                    let mut tempbuf: String = String::new();
                    ch = self.getch();
                    while ch != '"' {
                        tempbuf.push(ch);
                        ch = self.getch();
                    }
                    Token::Str(tempbuf)
                },
                '#' => {
                    while ch != '\n' {
                        ch = self.getch();
                    }
                    self.next_token()
                },
                _ => self.next_token(),
            }
        };
        return self.last.clone();
    }
}
// pub fn put_file_tokens(file_path: String) {
//     let contents = fs::read_to_string(file_path)
//         .expect("File does not exist");
//     let mut lexer: Lexer = Lexer{
//         buf: contents.chars().collect(),
//         ptr: 0,
//         last: Token::FileEnd,
//     };
//     let mut tok: Token = lexer.next_token();
//     while tok != Token::FileEnd {
//         println!("{:?}", tok);
//         tok = lexer.next_token();
//     }
// }
