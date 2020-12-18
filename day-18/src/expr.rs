#[derive(Clone, Eq, PartialEq)]
pub enum Expr {
    Number(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn evaluate(&self) -> i64 {
        match self {
            Expr::Number(n) => *n,
            Expr::Add(e1, e2) => e1.evaluate() + e2.evaluate(),
            Expr::Mul(e1, e2) => e1.evaluate() * e2.evaluate(),
        }
    }

    pub fn number(n: i64) -> Expr {
        Expr::Number(n)
    }

    pub fn add(e1: Expr, e2: Expr) -> Expr {
        let e1 = Box::new(e1);
        let e2 = Box::new(e2);
        Expr::Add(e1, e2)
    }

    pub fn mul(e1: Expr, e2: Expr) -> Expr {
        let e1 = Box::new(e1);
        let e2 = Box::new(e2);
        Expr::Mul(e1, e2)
    }
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(n) => write!(f, "{}", n),
            Expr::Add(e1, e2) => write!(f, "({:?} + {:?})", e1, e2),
            Expr::Mul(e1, e2) => write!(f, "({:?} * {:?})", e1, e2),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expr() {
        let expr = Expr::add(Expr::add(Expr::add(Expr::number(1), Expr::number(2)), Expr::number(3)), Expr::number(4));
        assert_eq!(expr.evaluate(), 10);
    }
}
