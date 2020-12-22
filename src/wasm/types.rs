//! Module defining types described in WebAssembly's specification.
use super::{indices, Encode};

#[derive(Clone, Debug)]
pub enum BlockType {
    Empty,
    Value(ValueType),
    Function(indices::TypeId),
}

#[derive(Clone, Debug)]
pub enum ValueType {
    I32,
    I64,
    F32,
    F64,
}

#[derive(Clone, Debug)]
pub struct ResultType {
    types: Vec<ValueType>,
}

#[derive(Clone, Debug)]
pub struct FunctionType {
    parameters: ResultType,
    results: ResultType,
}

#[derive(Clone, Debug)]
pub enum ReferenceType {
    Func,
}

#[derive(Clone, Debug)]
pub enum Limits {
    Min { min: u32 },
    MinMax { min: u32, max: u32 },
}

#[derive(Clone, Debug)]
pub struct MemoryType {
    limits: Limits,
}

#[derive(Clone, Debug)]
pub struct TableType {
    element_type: ReferenceType,
    limits: Limits,
}

#[derive(Clone, Debug)]
pub enum GlobalType {
    Const(ValueType),
    Var(ValueType),
}

impl ResultType {
    pub fn new(types: Vec<ValueType>) -> ResultType {
        ResultType { types }
    }
}

impl FunctionType {
    pub fn new(parameters: ResultType, results: ResultType) -> FunctionType {
        FunctionType {
            parameters,
            results,
        }
    }
}

impl MemoryType {
    pub fn new(limits: Limits) -> MemoryType {
        MemoryType { limits }
    }
}

impl TableType {
    pub fn new(element_type: ReferenceType, limits: Limits) -> TableType {
        TableType {
            element_type,
            limits,
        }
    }
}

impl Encode for BlockType {
    fn encode(&self) -> Vec<u8> {
        match self {
            BlockType::Empty => vec![0x40],
            BlockType::Value(value_type) => value_type.encode(),
            BlockType::Function(type_id) => type_id.encode_as_signed_u33(),
        }
    }
}

impl Encode for ValueType {
    fn encode(&self) -> Vec<u8> {
        match self {
            ValueType::I32 => vec![0x7f],
            ValueType::I64 => vec![0x7e],
            ValueType::F32 => vec![0x7d],
            ValueType::F64 => vec![0x7c],
        }
    }
}

impl Encode for ResultType {
    fn encode(&self) -> Vec<u8> {
        self.types.encode()
    }
}

impl Encode for FunctionType {
    fn encode(&self) -> Vec<u8> {
        let mut result = vec![0x60];
        result.extend_from_slice(&self.parameters.encode());
        result.extend_from_slice(&self.results.encode());
        result
    }
}

impl Encode for ReferenceType {
    fn encode(&self) -> Vec<u8> {
        match self {
            ReferenceType::Func => vec![0x70],
        }
    }
}

impl Encode for Limits {
    fn encode(&self) -> Vec<u8> {
        let mut result = Vec::new();

        match self {
            Limits::Min { min } => {
                result.push(0x00);
                leb128::write::unsigned(&mut result, *min as u64)
                    .expect("Failed to write LEB128 number");
            }
            Limits::MinMax { min, max } => {
                result.push(0x01);
                leb128::write::unsigned(&mut result, *min as u64)
                    .expect("Failed to write LEB128 number");
                leb128::write::unsigned(&mut result, *max as u64)
                    .expect("Failed to write LEB128 number");
            }
        }

        result
    }
}

impl Encode for MemoryType {
    fn encode(&self) -> Vec<u8> {
        self.limits.encode()
    }
}

impl Encode for TableType {
    fn encode(&self) -> Vec<u8> {
        let mut result = self.element_type.encode();
        result.extend_from_slice(&self.limits.encode());
        result
    }
}

impl Encode for GlobalType {
    fn encode(&self) -> Vec<u8> {
        match self {
            GlobalType::Const(value_type) => {
                let mut result = value_type.encode();
                result.push(0x00);
                result
            }
            GlobalType::Var(value_type) => {
                let mut result = value_type.encode();
                result.push(0x01);
                result
            }
        }
    }
}
