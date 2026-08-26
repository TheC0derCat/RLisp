enum Token {
    OpeningParen,
    ClosingParen,
    Identifier(String),
    Str(String),
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}
struct Lexer {
    buf: String,
    ptr: usize,
}
impl Lexer {
    fn getch(&mut self) -> char {
        let ch: char = self.buf.as_bytes()[self.ptr] as char;
        self.ptr += 1;
        return ch;
    }
    fn backch(&mut self){
        self.ptr -= 1;
    }
    fn next_token(&mut self) -> Token {
        let mut ch: char = self.getch();
        if ch.is_alphanumeric() {
            let mut tempbuf: String = String::new();
            while ch.is_alphanumeric() {
                tempbuf.push(ch);
                ch = self.getch();
            }
            match tempbuf.parse::<i32>() {
                Ok(n) => Token::Num(n),
                Err(e) => Token::Identifier(tempbuf),
            }
        }
        else {
            match ch {
                '+' => Token::Add,
                '-' => Token::Sub,
                '*' => Token::Mul,
                '/' => Token::Div,
                '%' => Token::Mod,
                _ => self.next_token(),
            }
        }
    }
}
fn main() {
    println!("Hello, world!");
}
