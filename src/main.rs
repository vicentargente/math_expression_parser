mod parser;

use parser::parser::Parser;

fn main() {
    //println!("{}", "-5".parse::<i32>().unwrap());
    loop {
        print!(">>> ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let input = input.trim().to_string();
        if input == "exit" {
            break;
        }
        if input == "" {
            continue;
        }

        let mut parser = Parser::new(input);
        let res = match parser.parse() {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        println!("{}", res);
    }
}