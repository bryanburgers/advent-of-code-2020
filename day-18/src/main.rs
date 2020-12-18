mod expr;
mod lexer;
mod parser;

use parser::{Parser, ParserPartB};

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut sum = 0;
    let parser = Parser;
    for line in input.lines() {
        let expr = parser.parse(line.trim());
        let val = expr.evaluate();
        sum += val;
    }
    println!("{}", sum);

    let mut sum = 0;
    let parser = ParserPartB;
    for line in input.lines() {
        let expr = parser.parse(line.trim());
        let val = expr.evaluate();
        sum += val;
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let parser = Parser;
        assert_eq!(parser.parse("1 + 2 * 3 + 4 * 5 + 6").evaluate(), 71);
        assert_eq!(parser.parse("1 + (2 * 3) + (4 * (5 + 6))").evaluate(), 51);
        assert_eq!(parser.parse("2 * 3 + (4 * 5)").evaluate(), 26);
        assert_eq!(parser.parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").evaluate(), 437);
        assert_eq!(
            parser
                .parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .evaluate(),
            12240
        );
        assert_eq!(
            parser
                .parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .evaluate(),
            13632
        );
    }

    #[test]
    fn examples_part_b() {
        let parser = ParserPartB;
        assert_eq!(parser.parse("1 + 2 * 3 + 4 * 5 + 6").evaluate(), 231);
        assert_eq!(parser.parse("1 + (2 * 3) + (4 * (5 + 6))").evaluate(), 51);
        assert_eq!(parser.parse("2 * 3 + (4 * 5)").evaluate(), 46);
        assert_eq!(parser.parse("5 + (8 * 3 + 9 + 3 * 4 * 3)").evaluate(), 1445);
        assert_eq!(
            parser
                .parse("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))")
                .evaluate(),
            669060
        );
        assert_eq!(
            parser
                .parse("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                .evaluate(),
            23340
        );
    }
}
