use super::Encode;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TypeId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FunctionId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TableId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MemoryId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GlobalId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LocalId {
    index: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LabelId {
    index: u32,
}

impl Encode for TypeId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for FunctionId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for TableId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for MemoryId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for GlobalId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for LocalId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for LabelId {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.index as u64)
            .expect("Failed to write LEB128 number");
        result
    }
}

impl Default for TableId {
    fn default() -> Self {
        TableId { index: 0x00 }
    }
}

impl Default for MemoryId {
    fn default() -> MemoryId {
        MemoryId { index: 0x00 }
    }
}

impl TypeId {
    pub fn encode_as_signed_u33(&self) -> Vec<u8> {
        let mut result = Vec::new();
        leb128::write::signed(&mut result, self.index as i64)
            .expect("Failed to write LEB128 number");
        result
    }

    pub fn new(index: u32) -> TypeId {
        TypeId { index }
    }
}

impl FunctionId {
    pub fn new(index: u32) -> FunctionId {
        FunctionId { index }
    }
}

impl TableId {
    pub fn new(index: u32) -> TableId {
        TableId { index }
    }
}

impl MemoryId {
    pub fn new(index: u32) -> MemoryId {
        MemoryId { index }
    }
}

impl GlobalId {
    pub fn new(index: u32) -> GlobalId {
        GlobalId { index }
    }
}

impl LocalId {
    pub fn new(index: u32) -> LocalId {
        LocalId { index }
    }
}

impl LabelId {
    pub fn new(index: u32) -> LabelId {
        LabelId { index }
    }
}
