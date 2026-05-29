use qnt::lexer::Lexer;

#[test]
fn main() {
    let text = "123";
    let tokens = Lexer::new(text).tokenize();

    match tokens {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token);
            }
        }
        Err(err) => {
            eprintln!("{}", err.report("test.qnt"));
        }
    }
}
