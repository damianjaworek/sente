use crate::{ast, wasm};

// fn emit_type_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x01;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let mut start_function_type = Vec::new();
//     start_function_type.push(0x60);
//     start_function_type.extend_from_slice(&wasm::encode_vector(&[]));
//     start_function_type.extend_from_slice(&wasm::encode_vector(&[]));

//     let function_types_vector = wasm::encode_nested_vector(&mut [start_function_type.into_iter()]);
//     let encoded_section = wasm::encode_vector(&function_types_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_import_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x02;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let imports_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&imports_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn emit_function_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x03;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let mut start_function_type_id = Vec::new();
//     leb128::write::unsigned(&mut start_function_type_id, 0).unwrap();

//     let types_vector = wasm::encode_nested_vector(&mut [start_function_type_id.into_iter()]);
//     let encoded_section = wasm::encode_vector(&types_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_table_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x04;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let tables_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&tables_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_memory_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x05;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let memories_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&memories_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_global_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x06;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let globals_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&globals_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_export_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x07;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let exports_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&exports_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn emit_start_section(start_function_id: u32) -> Vec<u8> {
//     const SECTION_ID: u8 = 0x08;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let mut encoded_start_function_id = Vec::new();
//     leb128::write::unsigned(&mut encoded_start_function_id, start_function_id as u64).unwrap();

//     section.extend_from_slice(&wasm::encode_vector(&encoded_start_function_id));
//     section
// }

// fn _emit_element_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x09;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let elements_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&elements_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn emit_code_section(stmts: &crate::ast::Stmts) -> Vec<u8> {
//     const SECTION_ID: u8 = 0x0a;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let mut start_function = Vec::new();
//     start_function.extend_from_slice(&wasm::encode_vector(&[]));
//     start_function.extend_from_slice(&compile_stmts(stmts));
//     start_function.push(wasm::opcodes::Opcode::End.into());

//     let start_function_code = wasm::encode_vector(&start_function);
//     let code_vector = wasm::encode_nested_vector(&mut [start_function_code.into_iter()]);
//     let encoded_section = wasm::encode_vector(&code_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

// fn _emit_data_section() -> Vec<u8> {
//     const SECTION_ID: u8 = 0x0b;
//     let mut section = Vec::new();
//     section.push(SECTION_ID);

//     let data_vector = wasm::encode_vector(&[]);
//     let encoded_section = wasm::encode_vector(&data_vector);
//     section.extend_from_slice(&encoded_section);

//     section
// }

fn compile_expr(expr: &crate::ast::Expression) -> Vec<Box<dyn wasm::instructions::Instruction>> {
    let mut output: Vec<Box<dyn wasm::instructions::Instruction>> = Vec::new();

    match expr {
        crate::ast::Expression::Number(number) => {
            let number = match number {
                ast::Number::Integer(int) => int.parse().unwrap(),
                ast::Number::Float(float) => float.parse::<f64>().unwrap() as i64,
            };

            output.push(Box::new(wasm::instructions::i64_const(number)));
        }
        crate::ast::Expression::Operation(left, operator, right) => {
            for instruction in compile_expr(left) {
                output.push(instruction);
            }

            for instruction in compile_expr(right) {
                output.push(instruction);
            }

            let instruction: Box<dyn wasm::instructions::Instruction> = match operator {
                crate::ast::Operator::Add => Box::new(wasm::instructions::i64_add()),
                crate::ast::Operator::Subtract => Box::new(wasm::instructions::i64_sub()),
                crate::ast::Operator::Multiply => Box::new(wasm::instructions::i64_mul()),
                crate::ast::Operator::Divide => Box::new(wasm::instructions::i64_div_s()),
            };

            output.push(instruction);
        }
    }

    output
}

fn compile_exprs(exprs: Vec<Box<ast::Expression>>) -> wasm::sections::Expression {
    let mut output = Vec::new();

    for expr in exprs {
        for instruction in compile_expr(&expr) {
            output.push(instruction);
        }
        output.push(Box::new(wasm::instructions::drop()));
    }

    output.push(Box::new(wasm::instructions::end()));

    wasm::sections::Expression::new(output)
}

pub fn emit(expressions: Vec<Box<ast::Expression>>) -> Vec<u8> {
    // I need to emit type section, function section, start section and code section
    // First emit Type section
    // Then emit Function section
    // Next emit Start section
    // Finally emit Code section

    let mut sections: Vec<Box<dyn wasm::sections::Section>> = Vec::new();

    // Hardcoded function type as () -> ()
    let empty_type = wasm::types::ResultType::new(Vec::new());
    let function_type = wasm::types::FunctionType::new(empty_type.clone(), empty_type);
    let type_section = wasm::sections::Type::new(vec![function_type]);
    sections.push(Box::new(type_section));

    // Hardcoded function as having type 0
    let type_id = wasm::indices::TypeId::new(0);
    let function_section = wasm::sections::Function::new(vec![type_id]);
    sections.push(Box::new(function_section));

    // Hardcoded start function id as 0
    let function_id = wasm::indices::FunctionId::new(0);
    let start_section = wasm::sections::Start::new(function_id);
    sections.push(Box::new(start_section));

    // Code section
    let compiled_expressions = compile_exprs(expressions);
    let code_entry = wasm::sections::CodeEntry::new(Vec::new(), compiled_expressions);
    let code_section = wasm::sections::Code::new(vec![code_entry]);
    sections.push(Box::new(code_section));

    // Module
    let module = wasm::module::Module::with(sections);

    module.encode()
}
