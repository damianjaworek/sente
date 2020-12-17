pub mod indices;
pub mod instructions;
pub mod module;
pub mod opcodes;
pub mod sections;
pub mod types;

pub trait Encode {
    fn encode(&self) -> Vec<u8>;
}

impl<T> Encode for Vec<T>
where
    T: Encode + std::fmt::Debug,
{
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, self.len() as u64)
            .expect("Failed to write LEB128 number");

        for item in self {
            result.extend_from_slice(&item.encode());
        }
        result
    }
}

impl Encode for i32 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::signed(&mut result, *self as i64).expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for i64 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::signed(&mut result, *self).expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for u32 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, *self as u64).expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for u64 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        let mut result = Vec::new();
        leb128::write::unsigned(&mut result, *self).expect("Failed to write LEB128 number");
        result
    }
}

impl Encode for f32 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        self.to_le_bytes().to_vec()
    }
}

impl Encode for f64 {
    fn encode(&self) -> Vec<u8> {
        dbg!(self);
        self.to_le_bytes().to_vec()
    }
}
