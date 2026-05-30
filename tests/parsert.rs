use qnt::lexer::Lexer;
use qnt::parser::Parser;

#[test]
fn expression() {
    let text = "a + 4 *-3
	4 < 6 && 7 + 4 > 3 && !true
	\"str\" + \"str2\"
	2 ^ 3 ^ 4
	2 + (4 + 4) * (8 + 8)
";
    println!("Source: {}", &text);
    let tokens = Lexer::new(text).tokenize();
    match tokens {
        Ok(tokens) => {
            let exprs = Parser::new(tokens, text).parse_expr();
            match exprs {
                Ok(expr) => {
                    for exp in expr {
                        println!("{}", exp);
                    }
                }
                Err(err) => eprintln!("{}", err.report("test.qnt")),
            }
        }
        Err(err) => {
            eprintln!("{}", err.report("test.qnt"));
        }
    }
}
