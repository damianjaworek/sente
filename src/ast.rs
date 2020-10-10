pub enum Expr {
    Number(i32),
    Op(Box<Expr>, Opcode, Box<Expr>),
}

pub enum Opcode {
    Mul,
    Div,
    Add,
    Sub,
}

impl std::fmt::Debug for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Expr::Number(number) => number.to_string(),
            Expr::Op(left, op, right) => format!("({:?}{:?}{:?})", left, op, right),
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Opcode::Mul => " * ",
            Opcode::Div => " / ",
            Opcode::Add => " + ",
            Opcode::Sub => " - ",
        };

        write!(f, "{}", value)
    }
}
