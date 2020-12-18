use crate::lexer::Lexer;
use crate::expr::Expr;

pub struct Parser;

impl Parser {
    pub fn parse(&self, input: &str) -> Expr {
        let lexer = Lexer::new(input);

        let mut stack = std::collections::VecDeque::new();

        for lexeme in lexer {
            match lexeme {
                "(" => {
                    stack.push_back(StackItem::Parenthesis);
                },
                ")" => {
                    let e = stack.pop_back();
                    let p = stack.pop_back();
                    assert_eq!(p, Some(StackItem::Parenthesis));
                    stack.push_back(e.unwrap());
                },
                "+" => {
                    stack.push_back(StackItem::Add);
                },
                "*" => {
                    stack.push_back(StackItem::Mul);
                },
                n => {
                    let number = lexeme.parse::<i64>().expect(&format!("Unexpected lexeme '{}'", n));
                    let expr = Expr::number(number);
                    stack.push_back(StackItem::Expr(expr));
                }
            }

            while stack.len() >= 3 && matches!(stack[stack.len() - 1], StackItem::Expr(..)) && matches!(stack[stack.len() - 2], StackItem::Add | StackItem::Mul) && matches!(stack[stack.len() - 3], StackItem::Expr(..)) {
                let e2 = stack.pop_back();
                let op = stack.pop_back();
                let e1 = stack.pop_back();

                let e1 = e1.unwrap().expr();
                let e2 = e2.unwrap().expr();
                let op = op.unwrap();

                let e = match op {
                    StackItem::Add => Expr::add(e1, e2),
                    StackItem::Mul => Expr::mul(e1, e2),
                    _ => unreachable!(),
                };

                stack.push_back(StackItem::Expr(e));
            }
        }

        stack.pop_back().unwrap().expr()
    }
}

#[derive(Debug, Eq, PartialEq)]
enum StackItem {
    Expr(Expr),
    Add,
    Mul,
    Parenthesis,
}

impl StackItem {
    fn expr(self) -> Expr {
        match self {
            StackItem::Expr(expr) => expr,
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = "1 + 2 * 3 + 4 * 5 + 6";
        let parser = Parser;
        let expr = parser.parse(input);

        assert_eq!(expr,
           Expr::add(
               Expr::mul(
                   Expr::add(
                       Expr::mul(
                           Expr::add(
                               Expr::number(1),
                               Expr::number(2)
                           ),
                           Expr::number(3)
                        ),
                        Expr::number(4)
                    ),
                    Expr::number(5)
                ),
                Expr::number(6)
            )
        );
    }

    #[test]
    fn second_example() {
        let input = "1 + (2 * 3) + (4 * (5 + 6))";
        let parser = Parser;
        let expr = parser.parse(input);

        let five_six = Expr::add(Expr::number(5), Expr::number(6));
        let four_five_six = Expr::mul(Expr::number(4), five_six);
        let two_three = Expr::mul(Expr::number(2), Expr::number(3));
        let one_two_three = Expr::add(Expr::number(1), two_three);
        let one_two_three_four_five_six = Expr::add(one_two_three, four_five_six);

        assert_eq!(expr, one_two_three_four_five_six);
    }
}
