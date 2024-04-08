use scanner::TokenKind;

use crate::scanner::Scanner;

mod scanner;

fn main() {
    let input: Vec<char> = r#"
    int main() {
        int test = 1 + 1;

        if (test == 2) {
            printf("Hello world");
        }
    }
"#
    .to_string()
    .chars()
    .collect();

    let mut scanner = Scanner::new(&input, "test.c".into());

    let mut token = scanner.scan_one();

    while token.kind != TokenKind::Eof {
        println!("{:?}", token.kind);
        token = scanner.scan_one();
    }
}
