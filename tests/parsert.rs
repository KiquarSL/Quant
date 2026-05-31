use colored::*;
use serial_test::serial;

use qnt::lexer::Lexer;
use qnt::parser::Parser;

#[test]
#[serial]
fn expression() {
    println!("{}", String::from("Expression").blue());
    let text = "a + 4 *-3
	4 < 6 && 7 + 4 > 3 && !true
	\"str\" + \"str2\"
	2 ^ 3 ^ 4
	2 + (4 + 4)  (8 + 8)
";
    println!("Source: \n{}", &text);
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

#[test]
#[serial]
fn statement() {
    println!("{}", String::from("Statement").blue());
    let text = "a: num = 5
b: 123 = true+";
    println!("Source: \n{}", &text);
    let tokens = Lexer::new(text).tokenize();
    match tokens {
        Ok(tokens) => {
            let stmts = Parser::new(tokens, text).parse_stmt();
            match stmts {
                Ok(stmt) => {
                    for stm in stmt {
                        println!("{:?}", stm);
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
