use qnt::lexer::Lexer;

#[test]
fn all_tokens() {
    let text = "+ - * / = ident() 123 true false!";
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
