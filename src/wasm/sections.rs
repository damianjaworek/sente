use super::{indices, instructions, types, Encode};

pub trait Section: Encode + std::fmt::Debug {}

#[derive(Debug)]
pub struct Type {
    types: Vec<types::FunctionType>,
}

#[derive(Debug)]
pub struct Import {
    imports: Vec<ImportEntry>,
}

#[derive(Debug)]
pub struct Function {
    functions: Vec<indices::TypeId>,
}

#[derive(Debug)]
pub struct Table {
    tables: Vec<types::TableType>,
}

#[derive(Debug)]
pub struct Memory {
    memories: Vec<types::MemoryType>,
}

#[derive(Debug)]
pub struct Global {
    globals: Vec<GlobalEntry>,
}

#[derive(Debug)]
pub struct Export {
    exports: Vec<ExportEntry>,
}

#[derive(Debug)]
pub struct Start {
    start: indices::FunctionId,
}

#[derive(Debug)]
pub struct Element {
    elements: Vec<ElementEntry>,
}

#[derive(Debug)]
pub struct Code {
    codes: Vec<CodeEntry>,
}

#[derive(Debug)]
pub struct Data {
    data: Vec<DataEntry>,
}

#[derive(Debug)]
pub struct CodeEntry {
    locals: Vec<Locals>,
    expression: Expression,
}

#[derive(Debug)]
pub struct Locals {
    n: u32,
    value_type: types::ValueType,
}

#[derive(Debug)]
pub struct Expression {
    instructions: Vec<Box<dyn instructions::Instruction>>,
}

#[derive(Debug)]
pub struct ImportEntry {
    module: Name,
    name: Name,
    import_description: ImportDescription,
}

#[derive(Debug)]
pub struct Name {
    name: String,
}

#[derive(Debug)]
pub enum ImportDescription {
    Function(super::indices::TypeId),
    Table(super::types::TableType),
    Memory(super::types::MemoryType),
    Global(super::types::GlobalType),
}

#[derive(Debug)]
pub struct GlobalEntry {
    global_type: types::GlobalType,
    expression: Expression,
}

#[derive(Debug)]
pub struct ExportEntry {
    name: Name,
    export_description: ExportDescription,
}

#[derive(Debug)]
pub enum ExportDescription {
    Function(super::indices::FunctionId),
    Table(super::indices::TableId),
    Memory(super::indices::MemoryId),
    Global(super::indices::GlobalId),
}

#[derive(Debug)]
pub struct ElementEntry {
    table_id: indices::TableId,
    offset: Expression,
    initialization: Vec<indices::FunctionId>,
}

#[derive(Debug)]
pub struct DataEntry {
    memory_id: indices::MemoryId,
    offset: Expression,
    initialization: Vec<u8>,
}

impl Type {
    const ID: u8 = 0x01;

    pub fn new(types: Vec<types::FunctionType>) -> Type {
        Type { types }
    }
}

impl Import {
    const ID: u8 = 0x02;

    pub fn new(imports: Vec<ImportEntry>) -> Import {
        Import { imports }
    }
}

impl Function {
    const ID: u8 = 0x03;

    pub fn new(functions: Vec<indices::TypeId>) -> Function {
        Function { functions }
    }
}

impl Table {
    const ID: u8 = 0x04;

    pub fn new(tables: Vec<types::TableType>) -> Table {
        Table { tables }
    }
}

impl Memory {
    const ID: u8 = 0x05;

    pub fn new(memories: Vec<types::MemoryType>) -> Memory {
        Memory { memories }
    }
}

impl Global {
    const ID: u8 = 0x06;

    pub fn new(globals: Vec<GlobalEntry>) -> Global {
        Global { globals }
    }
}

impl Export {
    const ID: u8 = 0x07;

    pub fn new(exports: Vec<ExportEntry>) -> Export {
        Export { exports }
    }
}

impl Start {
    const ID: u8 = 0x08;

    pub fn new(start: indices::FunctionId) -> Start {
        Start { start }
    }
}

impl Element {
    const ID: u8 = 0x09;

    pub fn new(elements: Vec<ElementEntry>) -> Element {
        Element { elements }
    }
}

impl Code {
    const ID: u8 = 0x0a;

    pub fn new(codes: Vec<CodeEntry>) -> Code {
        Code { codes }
    }
}

impl Data {
    const ID: u8 = 0x0b;

    pub fn new(data: Vec<DataEntry>) -> Data {
        Data { data }
    }
}

impl CodeEntry {
    pub fn new(locals: Vec<Locals>, expression: Expression) -> CodeEntry {
        CodeEntry { locals, expression }
    }
}

impl Locals {
    pub fn new(n: u32, value_type: types::ValueType) -> Locals {
        Locals { n, value_type }
    }
}

impl Expression {
    pub fn new(instructions: Vec<Box<dyn instructions::Instruction>>) -> Expression {
        Expression { instructions }
    }
}

impl ImportEntry {
    pub fn new(module: Name, name: Name, import_description: ImportDescription) -> ImportEntry {
        ImportEntry {
            module,
            name,
            import_description,
        }
    }
}

impl Name {
    pub fn new(name: String) -> Name {
        Name { name }
    }
}

impl GlobalEntry {
    pub fn new(global_type: types::GlobalType, expression: Expression) -> GlobalEntry {
        GlobalEntry {
            global_type,
            expression,
        }
    }
}

impl ExportEntry {
    pub fn new(name: Name, export_description: ExportDescription) -> ExportEntry {
        ExportEntry {
            name,
            export_description,
        }
    }
}

impl ElementEntry {
    pub fn new(
        table_id: indices::TableId,
        offset: Expression,
        initialization: Vec<indices::FunctionId>,
    ) -> ElementEntry {
        ElementEntry {
            table_id,
            offset,
            initialization,
        }
    }
}

impl DataEntry {
    pub fn new(
        memory_id: indices::MemoryId,
        offset: Expression,
        initialization: Vec<u8>,
    ) -> DataEntry {
        DataEntry {
            memory_id,
            offset,
            initialization,
        }
    }
}

impl super::Encode for CodeEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        result.extend_from_slice(&self.locals.encode());
        result.extend_from_slice(&self.expression.encode());

        let mut output = Vec::new();
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
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

impl super::Encode for ImportEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.module.encode());
        output.extend_from_slice(&self.name.encode());
        output.extend_from_slice(&self.import_description.encode());

        output
    }
}

impl super::Encode for Name {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.name.clone().into_bytes();

        let mut output = Vec::new();
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl super::Encode for ImportDescription {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        match self {
            ImportDescription::Function(type_id) => {
                let mut output = vec![0x00];
                output.extend_from_slice(&type_id.encode());
                output
            }
            ImportDescription::Table(table_type) => {
                let mut output = vec![0x01];
                output.extend_from_slice(&table_type.encode());
                output
            }
            ImportDescription::Memory(memory_type) => {
                let mut output = vec![0x02];
                output.extend_from_slice(&memory_type.encode());
                output
            }
            ImportDescription::Global(global_type) => {
                let mut output = vec![0x03];
                output.extend_from_slice(&global_type.encode());
                output
            }
        }
    }
}

impl Encode for GlobalEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.global_type.encode());
        output.extend_from_slice(&self.expression.encode());

        output
    }
}

impl Encode for ExportEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.name.encode());
        output.extend_from_slice(&self.export_description.encode());

        output
    }
}

impl Encode for ExportDescription {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        match self {
            ExportDescription::Function(function_id) => {
                let mut output = vec![0x00];
                output.extend_from_slice(&function_id.encode());
                output
            }
            ExportDescription::Table(table_id) => {
                let mut output = vec![0x01];
                output.extend_from_slice(&table_id.encode());
                output
            }
            ExportDescription::Memory(memory_id) => {
                let mut output = vec![0x02];
                output.extend_from_slice(&memory_id.encode());
                output
            }
            ExportDescription::Global(global_id) => {
                let mut output = vec![0x03];
                output.extend_from_slice(&global_id.encode());
                output
            }
        }
    }
}

impl Encode for ElementEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.table_id.encode());
        output.extend_from_slice(&self.offset.encode());
        output.extend_from_slice(&self.initialization.encode());

        output
    }
}

impl Encode for DataEntry {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut output = Vec::new();

        output.extend_from_slice(&self.memory_id.encode());
        output.extend_from_slice(&self.offset.encode());
        output.extend_from_slice(&self.initialization);

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
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Import {}

impl Encode for Import {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.imports.encode();

        let mut output = Vec::new();
        output.push(Import::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Function {}

impl Encode for Function {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.functions.encode();

        let mut output = Vec::new();
        output.push(Function::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Table {}

impl Encode for Table {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.tables.encode();

        let mut output = Vec::new();
        output.push(Table::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Memory {}

impl Encode for Memory {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.memories.encode();

        let mut output = Vec::new();
        output.push(Memory::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Global {}

impl Encode for Global {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.globals.encode();

        let mut output = Vec::new();
        output.push(Global::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Export {}

impl Encode for Export {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.exports.encode();

        let mut output = Vec::new();
        output.push(Export::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Start {}

impl Encode for Start {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = self.start.encode();

        let mut output = Vec::new();
        output.push(Start::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Element {}

impl Encode for Element {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = &self.elements.encode();

        let mut output = Vec::new();
        output.push(Element::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Code {}

impl Encode for Code {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = &self.codes.encode();

        let mut output = Vec::new();
        output.push(Code::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}

impl Section for Data {}

impl Encode for Data {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let result = &self.data.encode();

        let mut output = Vec::new();
        output.push(Data::ID);
        leb128::write::unsigned(&mut output, result.len() as u64)
            .expect("Failed to write LEB128 number");
        output.extend_from_slice(&result);
        output
    }
}
