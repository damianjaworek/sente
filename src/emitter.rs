//! Module defining functions used to emit WebAssembly code using AST.
use crate::{ast, services, wasm};

fn compile_number_expression(
    number: &ast::Number,
    _name_service: &mut services::NameService,
    _type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    match number {
        ast::Number::Integer(integer) => {
            let parsed_integer: i64 = integer
                .parse()
                .map_err(|_| format!("`{}` is not a correct 64 bit integer", integer))?;
            let i64_const = wasm::instructions::i64_const(parsed_integer);
            instructions.push(Box::new(i64_const));
        }
        ast::Number::Float(float) => {
            let parsed_float: f64 = float
                .parse()
                .map_err(|_| format!("`{}` is not a correct floating point number", float))?;
            let f64_const = wasm::instructions::f64_const(parsed_float);
            instructions.push(Box::new(f64_const));
        }
    }

    Ok(())
}

fn compile_variable_expression(
    variable_name: &str,
    name_service: &mut services::NameService,
    _type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    let local_id = name_service
        .find_local_variable_by_name(variable_name)
        .map(|v| v.get_local_id())
        .ok_or(format!(
            "Variable with name `{}` is not defined",
            variable_name
        ))?;

    let local_get = wasm::instructions::local_get(local_id);
    instructions.push(Box::new(local_get));

    Ok(())
}

fn convert_type_to(
    from: &ast::ExpressionType,
    to: &ast::ExpressionType,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    match (from, to) {
        (l, r) if l == r => Ok(()),
        (ast::ExpressionType::Int32, ast::ExpressionType::Int64) => {
            let i64_extend_i32_s = wasm::instructions::i64_extend_i32_s();
            instructions.push(Box::new(i64_extend_i32_s));
            Ok(())
        }
        (ast::ExpressionType::Int32, ast::ExpressionType::Float32) => {
            let f32_convert_i32_s = wasm::instructions::f32_convert_i32_s();
            instructions.push(Box::new(f32_convert_i32_s));
            Ok(())
        }
        (ast::ExpressionType::Int32, ast::ExpressionType::Float64) => {
            let f64_convert_i32_s = wasm::instructions::f64_convert_i32_s();
            instructions.push(Box::new(f64_convert_i32_s));
            Ok(())
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Int32) => {
            let i32_wrap_i64 = wasm::instructions::i32_wrap_i64();
            instructions.push(Box::new(i32_wrap_i64));
            Ok(())
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Float32) => {
            let f32_convert_i64_s = wasm::instructions::f32_convert_i64_s();
            instructions.push(Box::new(f32_convert_i64_s));
            Ok(())
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Float64) => {
            let f64_convert_i64_s = wasm::instructions::f64_convert_i64_s();
            instructions.push(Box::new(f64_convert_i64_s));
            Ok(())
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Int32) => {
            let i32_trunc_f32_s = wasm::instructions::i32_trunc_f32_s();
            instructions.push(Box::new(i32_trunc_f32_s));
            Ok(())
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Int64) => {
            let i64_trunc_f32_s = wasm::instructions::i64_trunc_f32_s();
            instructions.push(Box::new(i64_trunc_f32_s));
            Ok(())
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Float64) => {
            let f64_promote_f32 = wasm::instructions::f64_promote_f32();
            instructions.push(Box::new(f64_promote_f32));
            Ok(())
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Int32) => {
            let i32_trunc_f64_s = wasm::instructions::i32_trunc_f64_s();
            instructions.push(Box::new(i32_trunc_f64_s));
            Ok(())
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Int64) => {
            let i64_trunc_f64_s = wasm::instructions::i64_trunc_f64_s();
            instructions.push(Box::new(i64_trunc_f64_s));
            Ok(())
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Float32) => {
            let f32_demote_f64 = wasm::instructions::f32_demote_f64();
            instructions.push(Box::new(f32_demote_f64));
            Ok(())
        }
        _ => Err(format!(
            "Type conversion not supported: {:?} {:?}",
            from, to
        )),
    }
}

fn convert_types(
    left: &ast::Expression,
    right: &ast::Expression,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<ast::ExpressionType, String> {
    let left_type = left.get_type(name_service, type_service)?;
    let right_type = right.get_type(name_service, type_service)?;

    match (left_type.clone(), right_type.clone()) {
        (l, r) if l == r => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(left_type)
        }
        (ast::ExpressionType::Int32, ast::ExpressionType::Int64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let i64_extend_i32_s = wasm::instructions::i64_extend_i32_s();
            instructions.push(Box::new(i64_extend_i32_s));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Int32, ast::ExpressionType::Float32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let f32_convert_i32_s = wasm::instructions::f32_convert_i32_s();
            instructions.push(Box::new(f32_convert_i32_s));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Int32, ast::ExpressionType::Float64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let f64_convert_i32_s = wasm::instructions::f64_convert_i32_s();
            instructions.push(Box::new(f64_convert_i32_s));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Int32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let i64_extend_i32_s = wasm::instructions::i64_extend_i32_s();
            instructions.push(Box::new(i64_extend_i32_s));
            Ok(left_type)
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Float32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let f32_convert_i64_s = wasm::instructions::f32_convert_i64_s();
            instructions.push(Box::new(f32_convert_i64_s));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Int64, ast::ExpressionType::Float64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let f64_convert_i64_s = wasm::instructions::f64_convert_i64_s();
            instructions.push(Box::new(f64_convert_i64_s));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Int32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let f32_convert_i32_s = wasm::instructions::f32_convert_i32_s();
            instructions.push(Box::new(f32_convert_i32_s));
            Ok(left_type)
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Int64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let f32_convert_i64_s = wasm::instructions::f32_convert_i64_s();
            instructions.push(Box::new(f32_convert_i64_s));
            Ok(left_type)
        }
        (ast::ExpressionType::Float32, ast::ExpressionType::Float64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            let f64_promote_f32 = wasm::instructions::f64_promote_f32();
            instructions.push(Box::new(f64_promote_f32));
            compile_expression(right, name_service, type_service, instructions)?;
            Ok(right_type)
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Int32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let f64_convert_i32_s = wasm::instructions::f64_convert_i32_s();
            instructions.push(Box::new(f64_convert_i32_s));
            Ok(left_type)
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Int64) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let f64_convert_i64_s = wasm::instructions::f64_convert_i64_s();
            instructions.push(Box::new(f64_convert_i64_s));
            Ok(left_type)
        }
        (ast::ExpressionType::Float64, ast::ExpressionType::Float32) => {
            compile_expression(left, name_service, type_service, instructions)?;
            compile_expression(right, name_service, type_service, instructions)?;
            let f64_promote_f32 = wasm::instructions::f64_promote_f32();
            instructions.push(Box::new(f64_promote_f32));
            Ok(left_type)
        }
        _ => Err(format!(
            "Type conversion not supported: {:?} {:?}",
            left_type, right_type
        )),
    }
}

fn compile_operation_expression(
    left: &ast::Expression,
    operator: &ast::Operator,
    right: &ast::Expression,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    let operation_type = convert_types(left, right, name_service, type_service, instructions)?;

    match (operator, operation_type) {
        (ast::Operator::Add, ast::ExpressionType::Int32) => {
            let i32_add = wasm::instructions::i32_add();
            instructions.push(Box::new(i32_add));
        }
        (ast::Operator::Add, ast::ExpressionType::Int64) => {
            let i64_add = wasm::instructions::i64_add();
            instructions.push(Box::new(i64_add));
        }
        (ast::Operator::Add, ast::ExpressionType::Float32) => {
            let f32_add = wasm::instructions::f32_add();
            instructions.push(Box::new(f32_add));
        }
        (ast::Operator::Add, ast::ExpressionType::Float64) => {
            let f64_add = wasm::instructions::f64_add();
            instructions.push(Box::new(f64_add));
        }
        (ast::Operator::Subtract, ast::ExpressionType::Int32) => {
            let i32_sub = wasm::instructions::i32_sub();
            instructions.push(Box::new(i32_sub));
        }
        (ast::Operator::Subtract, ast::ExpressionType::Int64) => {
            let i64_sub = wasm::instructions::i64_sub();
            instructions.push(Box::new(i64_sub));
        }
        (ast::Operator::Subtract, ast::ExpressionType::Float32) => {
            let f32_sub = wasm::instructions::f32_sub();
            instructions.push(Box::new(f32_sub));
        }
        (ast::Operator::Subtract, ast::ExpressionType::Float64) => {
            let f64_sub = wasm::instructions::f64_sub();
            instructions.push(Box::new(f64_sub));
        }
        (ast::Operator::Multiply, ast::ExpressionType::Int32) => {
            let i32_mul = wasm::instructions::i32_mul();
            instructions.push(Box::new(i32_mul));
        }
        (ast::Operator::Multiply, ast::ExpressionType::Int64) => {
            let i64_mul = wasm::instructions::i64_mul();
            instructions.push(Box::new(i64_mul));
        }
        (ast::Operator::Multiply, ast::ExpressionType::Float32) => {
            let f32_mul = wasm::instructions::f32_mul();
            instructions.push(Box::new(f32_mul));
        }
        (ast::Operator::Multiply, ast::ExpressionType::Float64) => {
            let f64_mul = wasm::instructions::f64_mul();
            instructions.push(Box::new(f64_mul));
        }
        (ast::Operator::Divide, ast::ExpressionType::Int32) => {
            let i32_div_s = wasm::instructions::i32_div_s();
            instructions.push(Box::new(i32_div_s));
        }
        (ast::Operator::Divide, ast::ExpressionType::Int64) => {
            let i64_div_s = wasm::instructions::i64_div_s();
            instructions.push(Box::new(i64_div_s));
        }
        (ast::Operator::Divide, ast::ExpressionType::Float32) => {
            let f32_div = wasm::instructions::f32_div();
            instructions.push(Box::new(f32_div));
        }
        (ast::Operator::Divide, ast::ExpressionType::Float64) => {
            let f64_div = wasm::instructions::f64_div();
            instructions.push(Box::new(f64_div));
        }
    }

    Ok(())
}

fn compile_function_call(
    function_call: &ast::FunctionCallExpression,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    let function = name_service
        .find_function_by_name(&function_call.identifier)
        .ok_or(format!(
            "Function with name `{}` is not defined",
            function_call.identifier
        ))?;

    let function_type = type_service
        .find_type_by_id(function.get_type_id())
        .expect("Cannot find defined function type");

    let parameter_types = function_type.get_parameter_types();

    let mut argument_types = Vec::new();
    for (i, argument) in function_call.arguments.iter().enumerate() {
        let argument_type = argument.get_type(name_service, type_service)?;

        compile_expression(argument, name_service, type_service, instructions)?;

        if i >= parameter_types.len() {
            return Err(format!(
                "Too many arguments when calling function `{}`. Expected {}, got {}",
                function_call.identifier,
                parameter_types.len(),
                function_call.arguments.len(),
            ));
        }
        convert_type_to(
            &argument_type,
            &parameter_types[i].clone().into(),
            instructions,
        )?;
        argument_types.push(argument_type);
    }

    if argument_types.len() != parameter_types.len() {
        return Err(format!(
            "Wrong number of arguments when calling function `{}`. Expected {}, got {}",
            function_call.identifier,
            parameter_types.len(),
            function_call.arguments.len()
        ));
    }

    let call = wasm::instructions::call(function.get_function_id());
    instructions.push(Box::new(call));

    Ok(())
}

fn compile_expression(
    expression: &ast::Expression,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    match expression {
        ast::Expression::Number(number) => {
            compile_number_expression(number, name_service, type_service, instructions)?
        }
        ast::Expression::Variable(variable_name) => {
            compile_variable_expression(variable_name, name_service, type_service, instructions)?
        }
        ast::Expression::Operation(left, operator, right) => compile_operation_expression(
            left,
            operator,
            right,
            name_service,
            type_service,
            instructions,
        )?,
        ast::Expression::FunctionCall(function_call) => {
            compile_function_call(function_call, name_service, type_service, instructions)?
        }
    }

    Ok(())
}

fn compile_condition(
    condition: &ast::Condition,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    let operation_type = convert_types(
        &condition.left,
        &condition.right,
        name_service,
        type_service,
        instructions,
    )?;
    match (condition.comparison.clone(), operation_type) {
        (ast::ComparisonOperator::LessThan, ast::ExpressionType::Int32) => {
            let i32_lt_s = wasm::instructions::i32_lt_s();
            instructions.push(Box::new(i32_lt_s));
        }
        (ast::ComparisonOperator::LessThan, ast::ExpressionType::Int64) => {
            let i64_lt_s = wasm::instructions::i64_lt_s();
            instructions.push(Box::new(i64_lt_s));
        }
        (ast::ComparisonOperator::LessThan, ast::ExpressionType::Float32) => {
            let f32_lt = wasm::instructions::f32_lt();
            instructions.push(Box::new(f32_lt));
        }
        (ast::ComparisonOperator::LessThan, ast::ExpressionType::Float64) => {
            let f64_lt = wasm::instructions::f64_lt();
            instructions.push(Box::new(f64_lt));
        }
        (ast::ComparisonOperator::LessThanOrEqual, ast::ExpressionType::Int32) => {
            let i32_le_s = wasm::instructions::i32_le_s();
            instructions.push(Box::new(i32_le_s));
        }
        (ast::ComparisonOperator::LessThanOrEqual, ast::ExpressionType::Int64) => {
            let i64_le_s = wasm::instructions::i64_le_s();
            instructions.push(Box::new(i64_le_s));
        }
        (ast::ComparisonOperator::LessThanOrEqual, ast::ExpressionType::Float32) => {
            let f32_le = wasm::instructions::f32_le();
            instructions.push(Box::new(f32_le));
        }
        (ast::ComparisonOperator::LessThanOrEqual, ast::ExpressionType::Float64) => {
            let f64_le = wasm::instructions::f64_le();
            instructions.push(Box::new(f64_le));
        }
        (ast::ComparisonOperator::GreaterThan, ast::ExpressionType::Int32) => {
            let i32_gt_s = wasm::instructions::i32_gt_s();
            instructions.push(Box::new(i32_gt_s));
        }
        (ast::ComparisonOperator::GreaterThan, ast::ExpressionType::Int64) => {
            let i64_gt_s = wasm::instructions::i64_gt_s();
            instructions.push(Box::new(i64_gt_s));
        }
        (ast::ComparisonOperator::GreaterThan, ast::ExpressionType::Float32) => {
            let f32_gt = wasm::instructions::f32_gt();
            instructions.push(Box::new(f32_gt));
        }
        (ast::ComparisonOperator::GreaterThan, ast::ExpressionType::Float64) => {
            let f64_gt = wasm::instructions::f64_gt();
            instructions.push(Box::new(f64_gt));
        }
        (ast::ComparisonOperator::GreaterThanOrEqual, ast::ExpressionType::Int32) => {
            let i32_ge_s = wasm::instructions::i32_ge_s();
            instructions.push(Box::new(i32_ge_s));
        }
        (ast::ComparisonOperator::GreaterThanOrEqual, ast::ExpressionType::Int64) => {
            let i64_ge_s = wasm::instructions::i64_ge_s();
            instructions.push(Box::new(i64_ge_s));
        }
        (ast::ComparisonOperator::GreaterThanOrEqual, ast::ExpressionType::Float32) => {
            let f32_ge = wasm::instructions::f32_ge();
            instructions.push(Box::new(f32_ge));
        }
        (ast::ComparisonOperator::GreaterThanOrEqual, ast::ExpressionType::Float64) => {
            let f64_ge = wasm::instructions::f64_ge();
            instructions.push(Box::new(f64_ge));
        }
        (ast::ComparisonOperator::Equal, ast::ExpressionType::Int32) => {
            let i32_eq = wasm::instructions::i32_eq();
            instructions.push(Box::new(i32_eq));
        }
        (ast::ComparisonOperator::Equal, ast::ExpressionType::Int64) => {
            let i64_eq = wasm::instructions::i64_eq();
            instructions.push(Box::new(i64_eq));
        }
        (ast::ComparisonOperator::Equal, ast::ExpressionType::Float32) => {
            let f32_eq = wasm::instructions::f32_eq();
            instructions.push(Box::new(f32_eq));
        }
        (ast::ComparisonOperator::Equal, ast::ExpressionType::Float64) => {
            let f64_eq = wasm::instructions::f64_eq();
            instructions.push(Box::new(f64_eq));
        }
        (ast::ComparisonOperator::NotEqual, ast::ExpressionType::Int32) => {
            let i32_ne = wasm::instructions::i32_ne();
            instructions.push(Box::new(i32_ne));
        }
        (ast::ComparisonOperator::NotEqual, ast::ExpressionType::Int64) => {
            let i64_ne = wasm::instructions::i64_ne();
            instructions.push(Box::new(i64_ne));
        }
        (ast::ComparisonOperator::NotEqual, ast::ExpressionType::Float32) => {
            let f32_ne = wasm::instructions::f32_ne();
            instructions.push(Box::new(f32_ne));
        }
        (ast::ComparisonOperator::NotEqual, ast::ExpressionType::Float64) => {
            let f64_ne = wasm::instructions::f64_ne();
            instructions.push(Box::new(f64_ne));
        }
    }

    Ok(())
}

fn compile_binding_statement(
    binding: &ast::BindingStatement,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    name_service.define_local_variable(
        binding.identifier.clone(),
        binding.binding_type.clone().into(),
    )?;

    let local_id = name_service
        .find_local_variable_by_name(&binding.identifier)
        .expect("Cannot find defined variable")
        .get_local_id();

    let expression_type = binding.expression.get_type(name_service, type_service)?;

    compile_expression(
        &binding.expression,
        name_service,
        type_service,
        instructions,
    )?;

    convert_type_to(
        &expression_type,
        &binding.binding_type.clone().into(),
        instructions,
    )?;

    let local_set = wasm::instructions::local_set(local_id);
    instructions.push(Box::new(local_set));

    Ok(())
}

fn compile_assignment_statement(
    assignment: &ast::AssignmentStatement,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
) -> Result<(), String> {
    let local_variable = name_service
        .find_local_variable_by_name(&assignment.identifier)
        .ok_or(format!(
            "Variable with name `{}` is not defined",
            assignment.identifier
        ))?;

    let local_id = local_variable.get_local_id();

    let expression_type = assignment.expression.get_type(name_service, type_service)?;

    compile_expression(
        &assignment.expression,
        name_service,
        type_service,
        instructions,
    )?;

    convert_type_to(
        &expression_type,
        &local_variable.get_variable_type().into(),
        instructions,
    )?;

    let local_set = wasm::instructions::local_set(local_id);
    instructions.push(Box::new(local_set));

    Ok(())
}

fn compile_conditional_statement(
    conditional: &ast::ConditionalStatement,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
    type_id: wasm::indices::TypeId,
) -> Result<(), String> {
    compile_condition(
        &conditional.condition,
        name_service,
        type_service,
        instructions,
    )?;

    let if_instruction = wasm::instructions::if_instruction(wasm::types::BlockType::Empty);
    instructions.push(Box::new(if_instruction));

    let mut if_name_service = services::NameService::enclosed(name_service.clone());

    compile_statements(
        &conditional.consequence,
        &mut if_name_service,
        type_service,
        instructions,
        type_id,
    )?;

    let else_instruction = wasm::instructions::else_instruction();
    instructions.push(Box::new(else_instruction));

    name_service.next_local_id = if_name_service.next_local_id;
    let mut else_name_service = services::NameService::enclosed(name_service.clone());

    compile_statements(
        &conditional.alternative,
        &mut else_name_service,
        type_service,
        instructions,
        type_id,
    )?;

    name_service.next_local_id = else_name_service.next_local_id;

    let end_instruction = wasm::instructions::end();
    instructions.push(Box::new(end_instruction));

    Ok(())
}

fn compile_loop_statement(
    loop_statement: &ast::LoopStatement,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
    type_id: wasm::indices::TypeId,
) -> Result<(), String> {
    let block = wasm::instructions::block(wasm::types::BlockType::Empty);
    instructions.push(Box::new(block));

    let loop_instruction = wasm::instructions::loop_instruction(wasm::types::BlockType::Empty);
    instructions.push(Box::new(loop_instruction));

    compile_condition(
        &loop_statement.condition,
        name_service,
        type_service,
        instructions,
    )?;

    let i32_eqz = wasm::instructions::i32_eqz();
    instructions.push(Box::new(i32_eqz));

    let br_if = wasm::instructions::br_if(wasm::indices::LabelId::new(1));
    instructions.push(Box::new(br_if));

    let mut enclosed_name_service = services::NameService::enclosed(name_service.clone());

    compile_statements(
        &loop_statement.body,
        &mut enclosed_name_service,
        type_service,
        instructions,
        type_id,
    )?;

    let br = wasm::instructions::br(wasm::indices::LabelId::new(0));
    instructions.push(Box::new(br));

    name_service.next_local_id = enclosed_name_service.next_local_id;

    let end_loop = wasm::instructions::end();
    instructions.push(Box::new(end_loop));

    let end_block = wasm::instructions::end();
    instructions.push(Box::new(end_block));

    Ok(())
}

fn compile_return_statement(
    return_statement: &ast::ReturnStatement,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
    type_id: wasm::indices::TypeId,
) -> Result<(), String> {
    compile_expression(
        &return_statement.expression,
        name_service,
        type_service,
        instructions,
    )?;

    let expression_type = return_statement
        .expression
        .get_type(name_service, type_service)?;

    let return_type = type_service
        .find_type_by_id(type_id)
        .expect("Cannot find defined function type")
        .get_return_type();

    convert_type_to(&expression_type, &return_type.into(), instructions)?;

    let return_instruction = wasm::instructions::return_instruction();
    instructions.push(Box::new(return_instruction));

    Ok(())
}

fn compile_statements(
    statements: &[ast::Statement],
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
    instructions: &mut Vec<Box<dyn wasm::instructions::Instruction>>,
    type_id: wasm::indices::TypeId,
) -> Result<(), String> {
    for statement in statements.iter() {
        match statement {
            ast::Statement::Binding(binding) => {
                compile_binding_statement(binding, name_service, type_service, instructions)?
            }
            ast::Statement::Assignment(assignment) => {
                compile_assignment_statement(assignment, name_service, type_service, instructions)?
            }
            ast::Statement::Conditional(conditional) => compile_conditional_statement(
                conditional,
                name_service,
                type_service,
                instructions,
                type_id,
            )?,
            ast::Statement::Loop(loop_statement) => compile_loop_statement(
                loop_statement,
                name_service,
                type_service,
                instructions,
                type_id,
            )?,
            ast::Statement::Return(return_statement) => compile_return_statement(
                return_statement,
                name_service,
                type_service,
                instructions,
                type_id,
            )?,
        };
    }

    Ok(())
}

fn register_variables(
    statements: &[ast::Statement],
    name_service: &mut services::NameService,
) -> Result<(), String> {
    for statement in statements.iter() {
        match statement {
            ast::Statement::Binding(binding) => {
                name_service.define_local_variable(
                    binding.identifier.clone(),
                    binding.binding_type.clone().into(),
                )?;
            }
            ast::Statement::Conditional(conditional) => {
                register_variables(&conditional.consequence, name_service)?;
                register_variables(&conditional.alternative, name_service)?;
            }
            ast::Statement::Loop(loop_statement) => {
                register_variables(&loop_statement.body, name_service)?;
            }
            _ => (),
        }
    }

    Ok(())
}

fn compile_function(
    function: &ast::Function,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
) -> Result<wasm::sections::CodeEntry, String> {
    let parameter_types = function
        .parameters
        .iter()
        .map(|p| p.parameter_type.clone().into())
        .collect();
    let return_type = function.result_type.clone().into();
    let type_id = type_service.define_function_type(parameter_types, return_type);
    name_service.define_function(function.identifier.clone(), type_id)?;

    let mut variable_count_name_service = services::NameService::default();
    register_variables(&function.statements, &mut variable_count_name_service)?;

    let mut name_service = services::NameService::enclosed(name_service.clone());

    for parameter in function.parameters.iter() {
        name_service.define_local_variable(
            parameter.identifier.clone(),
            parameter.parameter_type.clone().into(),
        )?;
    }

    let mut instructions = Vec::new();
    compile_statements(
        &function.statements,
        &mut name_service,
        type_service,
        &mut instructions,
        type_id,
    )?;

    let emergency_return_value: Box<dyn wasm::instructions::Instruction> =
        match function.result_type {
            ast::Type::Int32 => Box::new(wasm::instructions::i32_const(0)),
            ast::Type::Int64 => Box::new(wasm::instructions::i64_const(0)),
            ast::Type::Float32 => Box::new(wasm::instructions::f32_const(0.0)),
            ast::Type::Float64 => Box::new(wasm::instructions::f64_const(0.0)),
        };
    instructions.push(emergency_return_value);
    let emergency_return = wasm::instructions::return_instruction();
    instructions.push(Box::new(emergency_return));

    let end = wasm::instructions::end();
    instructions.push(Box::new(end));
    let expression = wasm::sections::Expression::new(instructions);

    let locals = variable_count_name_service.get_locals();

    Ok(wasm::sections::CodeEntry::new(locals, expression))
}

fn compile_program(
    program: &ast::Program,
    name_service: &mut services::NameService,
    type_service: &mut services::TypeService,
) -> Result<wasm::module::Module, String> {
    let mut code_entries = Vec::new();

    for function in program.functions.iter() {
        code_entries.push(compile_function(&function, name_service, type_service)?);
    }

    let mut sections: Vec<Box<dyn wasm::sections::Section>> = Vec::new();

    let type_section = type_service.emit_type_section();
    sections.push(Box::new(type_section));

    let function_section = name_service.emit_function_section();
    sections.push(Box::new(function_section));

    let export_section = name_service.emit_export_section();
    sections.push(Box::new(export_section));

    let code_section = wasm::sections::Code::new(code_entries);
    sections.push(Box::new(code_section));

    Ok(wasm::module::Module::with(sections))
}

/// Emits WebAssembly code corresponding to the AST it receives as a parameter.
/// Returns vector of bytes representing binary WebAssembly module or an error.
/// Uses [services::NameService] and [services::TypeService].
pub fn emit(program: ast::Program) -> Result<Vec<u8>, String> {
    let mut name_service = services::NameService::default();
    let mut type_service = services::TypeService::default();

    let module = compile_program(&program, &mut name_service, &mut type_service)?;

    Ok(module.encode())
}
