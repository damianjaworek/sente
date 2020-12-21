#[derive(Clone)]
pub struct Program {
    pub functions: Vec<Function>,
}

#[derive(Clone)]
pub struct Function {
    pub parameters: Vec<Parameter>,
    pub result_type: Type,
    pub statements: Vec<Statement>,
    pub identifier: String,
}

#[derive(Clone)]
pub struct Parameter {
    pub parameter_type: Type,
    pub identifier: String,
}

#[derive(Clone)]
pub enum Statement {
    Binding(BindingStatement),
    Assignment(AssignmentStatement),
    Conditional(ConditionalStatement),
    Loop(LoopStatement),
    Return(ReturnStatement),
}

#[derive(Clone)]
pub struct BindingStatement {
    pub binding_type: Type,
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Clone)]
pub struct AssignmentStatement {
    pub identifier: String,
    pub expression: Box<Expression>,
}

#[derive(Clone)]
pub struct ConditionalStatement {
    pub condition: Box<Condition>,
    pub consequence: Vec<Statement>,
    pub alternative: Vec<Statement>,
}

#[derive(Clone)]
pub struct LoopStatement {
    pub condition: Box<Condition>,
    pub body: Vec<Statement>,
}

#[derive(Clone)]
pub struct ReturnStatement {
    pub expression: Box<Expression>,
}

#[derive(Clone)]
pub struct Condition {
    pub left: Box<Expression>,
    pub comparison: ComparisonOperator,
    pub right: Box<Expression>,
}

#[derive(Clone)]
pub enum ComparisonOperator {
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Clone)]
pub enum Type {
    Int32,
    Int64,
    Float32,
    Float64,
}

#[derive(Clone)]
pub enum Expression {
    Number(Number),
    Operation(Box<Expression>, Operator, Box<Expression>),
    Variable(String),
    FunctionCall(FunctionCallExpression),
}

#[derive(Clone)]
pub struct FunctionCallExpression {
    pub identifier: String,
    #[allow(warnings)]
    pub arguments: Vec<Box<Expression>>,
}

#[derive(Clone)]
pub enum Operator {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Clone)]
pub enum Number {
    Integer(String),
    Float(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionType {
    Int32,
    Int64,
    Float32,
    Float64,
}

impl Program {
    pub fn with(functions: Vec<Function>) -> Program {
        Program { functions }
    }
}

impl Function {
    pub fn new(
        parameters: Vec<Parameter>,
        result_type: Type,
        statements: Vec<Statement>,
        identifier: String,
    ) -> Function {
        Function {
            parameters,
            result_type,
            statements,
            identifier,
        }
    }
}

impl Parameter {
    pub fn new(parameter_type: Type, identifier: String) -> Parameter {
        Parameter {
            parameter_type,
            identifier,
        }
    }
}

impl BindingStatement {
    pub fn new(
        binding_type: Type,
        identifier: String,
        expression: Box<Expression>,
    ) -> BindingStatement {
        BindingStatement {
            binding_type,
            identifier,
            expression,
        }
    }
}

impl AssignmentStatement {
    pub fn new(identifier: String, expression: Box<Expression>) -> AssignmentStatement {
        AssignmentStatement {
            identifier,
            expression,
        }
    }
}

impl ConditionalStatement {
    pub fn new(
        condition: Box<Condition>,
        consequence: Vec<Statement>,
        alternative: Vec<Statement>,
    ) -> ConditionalStatement {
        ConditionalStatement {
            condition,
            consequence,
            alternative,
        }
    }
}

impl LoopStatement {
    pub fn new(condition: Box<Condition>, body: Vec<Statement>) -> LoopStatement {
        LoopStatement { condition, body }
    }
}

impl ReturnStatement {
    pub fn new(expression: Box<Expression>) -> ReturnStatement {
        ReturnStatement { expression }
    }
}

impl Expression {
    pub fn get_type(
        &self,
        name_service: &super::services::NameService,
        type_service: &super::services::TypeService,
    ) -> Result<ExpressionType, String> {
        match self {
            Expression::Number(Number::Integer(_)) => Ok(ExpressionType::Int64),
            Expression::Number(Number::Float(_)) => Ok(ExpressionType::Float64),
            Expression::Variable(variable) => name_service
                .find_local_variable_by_name(variable)
                .map(|v| v.get_variable_type().into())
                .ok_or(format!("Variable with name `{}` is not defined", variable)),
            Expression::FunctionCall(function_call) => {
                let type_id = name_service
                    .find_function_by_name(&function_call.identifier)
                    .map(|f| f.get_type_id())
                    .ok_or(format!(
                        "Function with name `{}` is not defined",
                        function_call.identifier
                    ))?;
                let function_type = type_service
                    .find_type_by_id(type_id)
                    .expect("Cannot find defined type");
                Ok(function_type.get_return_type().into())
            }
            Expression::Operation(left, _, right) => {
                let left_type = left.get_type(name_service, type_service)?;
                let right_type = right.get_type(name_service, type_service)?;

                Ok(match (left_type, right_type) {
                    (l, r) if l == r => l,
                    (ExpressionType::Int32, r) => r,
                    (l, ExpressionType::Int32) => l,
                    (ExpressionType::Int64, r) => r,
                    (l, ExpressionType::Int64) => l,
                    (ExpressionType::Float32, r) => r,
                    (l, ExpressionType::Float32) => l,
                    (ExpressionType::Float64, r) => r,
                })
            }
        }
    }
}

impl Condition {
    pub fn new(
        left: Box<Expression>,
        comparison: ComparisonOperator,
        right: Box<Expression>,
    ) -> Condition {
        Condition {
            left,
            comparison,
            right,
        }
    }
}

impl FunctionCallExpression {
    #[allow(warnings)]
    pub fn new(identifier: String, arguments: Vec<Box<Expression>>) -> FunctionCallExpression {
        FunctionCallExpression {
            identifier,
            arguments,
        }
    }
}

impl From<Type> for ExpressionType {
    fn from(t: Type) -> Self {
        match t {
            Type::Int32 => ExpressionType::Int32,
            Type::Int64 => ExpressionType::Int64,
            Type::Float32 => ExpressionType::Float32,
            Type::Float64 => ExpressionType::Float64,
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
            "fn {:?} ({:?}) -> {:?} {{ \n{:?}}}",
            self.identifier, self.parameters, self.result_type, self.statements
        );

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Parameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("({:?}): {:?}\n", self.identifier, self.parameter_type);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Statement::Binding(binding) => format!("{:?};\n", binding),
            Statement::Assignment(assignment) => format!("{:?};\n", assignment),
            Statement::Conditional(conditional) => format!("{:?}\n", conditional),
            Statement::Loop(loop_statement) => format!("{:?}\n", loop_statement),
            Statement::Return(return_statement) => format!("{:?};\n", return_statement),
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for ComparisonOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            ComparisonOperator::LessThan => "<",
            ComparisonOperator::LessThanOrEqual => "<=",
            ComparisonOperator::GreaterThan => ">",
            ComparisonOperator::GreaterThanOrEqual => ">=",
            ComparisonOperator::Equal => "==",
            ComparisonOperator::NotEqual => "!=",
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?} {:?} {:?}", self.left, self.comparison, self.right);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("return {:?}", self.expression);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for LoopStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("while ({:?}) {{{:?}}}", self.condition, self.body);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for ConditionalStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!(
            "if ({:?}) {{{:?}}} else {{{:?}}}",
            self.condition, self.consequence, self.alternative
        );

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for AssignmentStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?} = {:?}", self.identifier, self.expression);

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for BindingStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!(
            "var {:?}: {:?} = {:?}",
            self.identifier, self.binding_type, self.expression
        );

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
            Expression::Variable(identifier) => format!("{:?}", identifier),
            Expression::FunctionCall(function_call) => format!("{:?}", function_call),
        };

        write!(f, "{}", value)
    }
}

impl std::fmt::Debug for FunctionCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = format!("{:?}({:?})", self.identifier, self.arguments);

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
