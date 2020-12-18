#[derive(Clone, Copy, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    idx: usize,
    state: State,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum State {
    Initial,
    InNumber { start: usize },
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            input,
            idx: 0,
            state: State::Initial,
        }
    }

    fn peek(&self) -> Option<&'a str> {
        if self.idx < self.input.len() {
            let ch = &self.input[self.idx..self.idx + 1];
            Some(ch)
        } else {
            None
        }
    }

    fn advance(&mut self) {
        self.idx += 1;
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.peek() {
                None => match self.state {
                    State::Initial => {
                        break None;
                    }
                    State::InNumber { start } => {
                        let r = &self.input[start..];
                        self.state = State::Initial;
                        break Some(r);
                    }
                },
                Some(" ") => match self.state {
                    State::Initial => {
                        self.advance();
                        continue;
                    }
                    State::InNumber { start } => {
                        let r = &self.input[start..self.idx];
                        self.state = State::Initial;
                        self.advance();
                        break Some(r);
                    }
                },
                Some("0") | Some("1") | Some("2") | Some("3") | Some("4") | Some("5")
                | Some("6") | Some("7") | Some("8") | Some("9") => match self.state {
                    State::Initial => {
                        self.state = State::InNumber { start: self.idx };
                        self.advance();
                        continue;
                    }
                    State::InNumber { .. } => {
                        self.advance();
                        continue;
                    }
                },
                Some("+") | Some("*") | Some("(") | Some(")") => match self.state {
                    State::InNumber { start } => {
                        let r = &self.input[start..self.idx];
                        self.state = State::Initial;
                        break Some(r);
                    }
                    State::Initial => {
                        let r = &self.input[self.idx..self.idx + 1];
                        self.advance();
                        break Some(r);
                    }
                },

                Some(s) => {
                    panic!("Unexpected '{}'", s);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lexer() {
        let lexer = Lexer::new("5 + 23 * ((19 + 2) * 73)");
        let lexemes: Vec<&str> = lexer.collect();
        assert_eq!(
            lexemes,
            vec!["5", "+", "23", "*", "(", "(", "19", "+", "2", ")", "*", "73", ")"]
        );
    }

    #[test]
    fn multi_digit_number_at_end() {
        let lexer = Lexer::new("5 + 23");
        let lexemes: Vec<&str> = lexer.collect();
        assert_eq!(lexemes, vec!["5", "+", "23"]);
    }

    #[test]
    fn no_spaces() {
        let lexer = Lexer::new("5+23*((19+2)*73)");
        let lexemes: Vec<&str> = lexer.collect();
        assert_eq!(
            lexemes,
            vec!["5", "+", "23", "*", "(", "(", "19", "+", "2", ")", "*", "73", ")"]
        );
    }
}
