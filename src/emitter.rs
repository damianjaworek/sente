const MAGIC_MODULE_HEADER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
const MODULE_VERSION: [u8; 4] = [0x01, 0x00, 0x00, 0x00];

use crate::wasm;

fn emit_type_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x01;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let mut start_function_type = Vec::new();
    start_function_type.push(0x60);
    start_function_type.extend_from_slice(&wasm::encode_vector(&[]));
    start_function_type.extend_from_slice(&wasm::encode_vector(&[]));

    let function_types_vector = wasm::encode_nested_vector(&mut [start_function_type.into_iter()]);
    let encoded_section = wasm::encode_vector(&function_types_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_import_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x02;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let imports_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&imports_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn emit_function_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x03;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let mut start_function_type_id = Vec::new();
    leb128::write::unsigned(&mut start_function_type_id, 0).unwrap();

    let types_vector = wasm::encode_nested_vector(&mut [start_function_type_id.into_iter()]);
    let encoded_section = wasm::encode_vector(&types_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_table_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x04;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let tables_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&tables_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_memory_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x05;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let memories_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&memories_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_global_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x06;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let globals_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&globals_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_export_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x07;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let exports_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&exports_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn emit_start_section(start_function_id: u32) -> Vec<u8> {
    const SECTION_ID: u8 = 0x08;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let mut encoded_start_function_id = Vec::new();
    leb128::write::unsigned(&mut encoded_start_function_id, start_function_id as u64).unwrap();

    section.extend_from_slice(&wasm::encode_vector(&encoded_start_function_id));
    section
}

fn _emit_element_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x09;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let elements_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&elements_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn emit_code_section(stmts: &crate::ast::Stmts) -> Vec<u8> {
    const SECTION_ID: u8 = 0x0a;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let mut start_function = Vec::new();
    start_function.extend_from_slice(&wasm::encode_vector(&[]));
    start_function.extend_from_slice(&compile_stmts(stmts));
    start_function.push(wasm::opcodes::Opcode::End.into());

    let start_function_code = wasm::encode_vector(&start_function);
    let code_vector = wasm::encode_nested_vector(&mut [start_function_code.into_iter()]);
    let encoded_section = wasm::encode_vector(&code_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn _emit_data_section() -> Vec<u8> {
    const SECTION_ID: u8 = 0x0b;
    let mut section = Vec::new();
    section.push(SECTION_ID);

    let data_vector = wasm::encode_vector(&[]);
    let encoded_section = wasm::encode_vector(&data_vector);
    section.extend_from_slice(&encoded_section);

    section
}

fn compile_expr(expr: &crate::ast::Expr) -> Vec<u8> {
    let mut output = Vec::new();

    match expr {
        crate::ast::Expr::Number(number) => {
            output.push(wasm::opcodes::Opcode::I32Const.into());
            leb128::write::signed(&mut output, *number as i64).unwrap();
        }
        crate::ast::Expr::Op(left, op, right) => {
            output.extend_from_slice(&compile_expr(left));
            output.extend_from_slice(&compile_expr(right));

            let opcode = match op {
                crate::ast::Opcode::Add => wasm::opcodes::Opcode::I32Add,
                crate::ast::Opcode::Sub => wasm::opcodes::Opcode::I32Sub,
                crate::ast::Opcode::Mul => wasm::opcodes::Opcode::I32Mul,
                crate::ast::Opcode::Div => wasm::opcodes::Opcode::I32DivS,
            };

            output.push(opcode.into());
        }
    }

    output
}

fn compile_stmts(stmts: &crate::ast::Stmts) -> Vec<u8> {
    let mut output = Vec::new();

    for expr in &stmts.stmts {
        output.extend_from_slice(&compile_expr(&expr));
        output.push(wasm::opcodes::Opcode::Drop.into());
    }

    output
}

pub fn emit(ast: &crate::ast::Stmts) -> Vec<u8> {
    let mut output = Vec::new();
    output.extend_from_slice(&MAGIC_MODULE_HEADER);
    output.extend_from_slice(&MODULE_VERSION);

    output.extend_from_slice(&emit_type_section());
    // output.extend_from_slice(&emit_import_section());
    output.extend_from_slice(&emit_function_section());
    // output.extend_from_slice(&emit_table_section());
    // output.extend_from_slice(&emit_memory_section());
    // output.extend_from_slice(&emit_global_section());
    // output.extend_from_slice(&emit_export_section());
    output.extend_from_slice(&emit_start_section(0));
    // output.extend_from_slice(&emit_element_section());
    output.extend_from_slice(&emit_code_section(ast));
    // output.extend_from_slice(&emit_data_section());

    output
}
