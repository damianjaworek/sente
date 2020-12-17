pub struct Program {
    functions: Vec<Function>,
}

pub struct Function {
    parameter_types: Vec<Type>,
    result_type: Type,
    statements: Vec<Statement>,
}

pub enum Statement {
    Expression(Expression),
}

pub enum Type {
    Int32,
    Int64,
    Float32,
    Float64,
}

pub enum Expression {
    Number(Number),
    Operation(Box<Expression>, Operator, Box<Expression>),
}

pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
}

pub enum Number {
    Integer(String),
    Float(String),
}

impl Program {
    pub fn with(functions: Vec<Function>) -> Program {
        Program { functions }
    }
}

impl Function {
    pub fn new(
        parameter_types: Vec<Type>,
        result_type: Type,
        statements: Vec<Statement>,
    ) -> Function {
        Function {
            parameter_types,
            result_type,
            statements,
        }
    }
}

impl std::fmt::Debug for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{{\n{:?}}}", self.functions);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!(
            "({:?}) -> {:?} {{ \n{:?}}}",
            self.parameter_types, self.result_type, self.statements
        );

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::Expression(expression) => format!("{:?};\n", expression),
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Type::Int32 => "Int32",
            Type::Int64 => "Int64",
            Type::Float32 => "Float32",
            Type::Float64 => "Float64",
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Expression::Number(number) => format!("{:?}", number),
            Expression::Operation(left, op, right) => format!("({:?}{:?}{:?})", left, op, right),
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Operator::Multiply => " * ",
            Operator::Divide => " / ",
            Operator::Add => " + ",
            Operator::Subtract => " - ",
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Number::Integer(number) => format!("Int({:?})", number),
            Number::Float(number) => format!("Float({:?})", number),
        };

        write!(f, "{}", value)
    }
}
