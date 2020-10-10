const MAGIC_MODULE_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const MODULE_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

pub fn emit() -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(&MAGIC_MODULE_HEADER);
    output.extend_from_slice(&MODULE_VERSION);
    output
}
