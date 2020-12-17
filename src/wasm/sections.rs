use super::{indices, instructions, types, Encode};

pub trait Section: Encode + std::fmt::Debug {}

#[derive(Debug)]
pub struct Type {
    types: Vec<types::FunctionType>,
}

#[derive(Debug)]
pub struct Import {}

#[derive(Debug)]
pub struct Function {
    functions: Vec<indices::TypeId>,
}

#[derive(Debug)]
pub struct Table {}

#[derive(Debug)]
pub struct Memory {}

#[derive(Debug)]
pub struct Global {}

#[derive(Debug)]
pub struct Export {}

#[derive(Debug)]
pub struct Start {
    start: indices::FunctionId,
}

#[derive(Debug)]
pub struct Element {}

#[derive(Debug)]
pub struct Code {
    codes: Vec<CodeEntry>,
}

#[derive(Debug)]
pub struct CodeEntry {
    locals: Vec<Locals>,
    expression: Expression,
}

#[derive(Debug)]
// TODO: understand what is it
pub struct Locals {
    n: u32,
    value_type: types::ValueType,
}

#[derive(Debug)]
pub struct Expression {
    instructions: Vec<Box<dyn instructions::Instruction>>,
}

#[derive(Debug)]
pub struct Data {}

impl Type {
    const ID: u8 = 0x01;

    pub fn new(types: Vec<types::FunctionType>) -> Type {
        Type { types }
    }
}

impl Import {
    const ID: u8 = 0x02;
}

impl Function {
    const ID: u8 = 0x03;

    pub fn new(functions: Vec<indices::TypeId>) -> Function {
        Function { functions }
    }
}

impl Table {
    const ID: u8 = 0x04;
}

impl Memory {
    const ID: u8 = 0x05;
}

impl Global {
    const ID: u8 = 0x06;
}

impl Export {
    const ID: u8 = 0x07;
}

impl Start {
    const ID: u8 = 0x08;

    pub fn new(start: indices::FunctionId) -> Start {
        Start { start }
    }
}

impl Element {
    const ID: u8 = 0x09;
}

impl Code {
    const ID: u8 = 0x0a;

    pub fn new(codes: Vec<CodeEntry>) -> Code {
        Code { codes }
    }
}

impl Data {
    const ID: u8 = 0x0b;
}

impl CodeEntry {
    pub fn new(locals: Vec<Locals>, expression: Expression) -> CodeEntry {
        CodeEntry { locals, expression }
    }
}

impl Expression {
    pub fn new(instructions: Vec<Box<dyn instructions::Instruction>>) -> Expression {
        Expression { instructions }
    }
}

impl super::Encode for CodeEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        result.extend_from_slice(&self.locals.encode());
        result.extend_from_slice(&self.expression.encode());

        let mut output = Vec::new();
        leb128::write::unsigned(&mut output, result.len() as u64).unwrap();
        output.extend_from_slice(&result);
        output
    }
}

impl super::Encode for Expression {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        for instruction in &self.instructions {
            output.extend_from_slice(&instruction.encode());
        }

        // Or demand that there is an end instruction in the instructions
        //output.extend_from_slice(&super::instructions::end().encode());

        output
    }
}

impl super::Encode for Locals {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.n.encode());
        output.extend_from_slice(&self.value_type.encode());

        output
    }
}

impl Section for Type {}

impl Encode for Type {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.types.encode();

        let mut output = Vec::new();
        output.push(Type::ID);
        leb128::write::unsigned(&mut output, result.len() as u64).unwrap();
        output.extend_from_slice(&result);
        output
    }
}

// impl Section for Import {}

impl Section for Function {}

impl Encode for Function {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.functions.encode();

        let mut output = Vec::new();
        output.push(Function::ID);
        leb128::write::unsigned(&mut output, result.len() as u64).unwrap();
        output.extend_from_slice(&result);
        output
    }
}

// impl Section for Table {}

// impl Section for Memory {}

// impl Section for Global {}

// impl Section for Export {}

impl Section for Start {}

impl Encode for Start {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.start.encode();

        let mut output = Vec::new();
        output.push(Start::ID);
        leb128::write::unsigned(&mut output, result.len() as u64).unwrap();
        output.extend_from_slice(&result);
        output
    }
}

// impl Section for Element {}

impl Section for Code {}

impl Encode for Code {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = &self.codes.encode();

        let mut output = Vec::new();
        output.push(Code::ID);
        leb128::write::unsigned(&mut output, result.len() as u64).unwrap();
        output.extend_from_slice(&result);
        output
    }
}

// impl Section for Data {}
