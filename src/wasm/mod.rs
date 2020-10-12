pub mod instructions;
pub mod opcodes;

pub fn encode_vector(vector: &[u8]) -> Vec<u8> {
    let mut encoded_vector = Vec::new();
    leb128::write::unsigned(&mut encoded_vector, vector.len() as u64).unwrap();
    encoded_vector.extend_from_slice(vector);
    encoded_vector
}

pub fn encode_nested_vector<T>(vector: &mut [T]) -> Vec<u8>
where
    T: Iterator<Item = u8>,
{
    let mut encoded_vector = Vec::new();
    leb128::write::unsigned(&mut encoded_vector, vector.len() as u64).unwrap();
    let flattened = vector.iter_mut().flatten().collect::<Vec<u8>>();
    encoded_vector.extend_from_slice(&flattened);
    encoded_vector
}
