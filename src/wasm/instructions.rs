use super::opcodes::Opcode;

pub fn unreachable() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Unreachable,
    }
}

pub fn nop() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Nop,
    }
}

pub fn block(block_type: BlockType) -> impl Instruction {
    BlockInstruction {
        opcode: Opcode::Block,
        block_type,
    }
}

pub fn loop_instruction(block_type: BlockType) -> impl Instruction {
    BlockInstruction {
        opcode: Opcode::Loop,
        block_type,
    }
}

pub fn if_instruction(block_type: BlockType) -> impl Instruction {
    BlockInstruction {
        opcode: Opcode::If,
        block_type,
    }
}

pub fn else_instruction() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Else,
    }
}

pub fn end() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::End,
    }
}

pub fn br(label_index: LabelIndex) -> impl Instruction {
    LabelInstruction {
        opcode: Opcode::Br,
        label_index,
    }
}

pub fn br_if(label_index: LabelIndex) -> impl Instruction {
    LabelInstruction {
        opcode: Opcode::BrIf,
        label_index,
    }
}

pub fn br_table(labels_vector: Vec<LabelIndex>, label_index: LabelIndex) -> impl Instruction {
    IndirectLabelInstruction {
        opcode: Opcode::BrTable,
        labels_vector,
        label_index,
    }
}

pub fn return_instruction() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Return,
    }
}

pub fn call(function_index: FunctionIndex) -> impl Instruction {
    FunctionInstruction {
        opcode: Opcode::Call,
        function_index,
    }
}

pub fn call_indirect(type_index: TypeIndex) -> impl Instruction {
    FunctionIndirectInstruction {
        opcode: Opcode::CallIndirect,
        type_index,
    }
}

pub fn drop() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Drop,
    }
}

pub fn select() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::Select,
    }
}

pub fn local_get(local_index: LocalIndex) -> impl Instruction {
    LocalInstruction {
        opcode: Opcode::LocalGet,
        local_index,
    }
}

pub fn local_set(local_index: LocalIndex) -> impl Instruction {
    LocalInstruction {
        opcode: Opcode::LocalSet,
        local_index,
    }
}

pub fn local_tee(local_index: LocalIndex) -> impl Instruction {
    LocalInstruction {
        opcode: Opcode::LocalTee,
        local_index,
    }
}

pub fn global_get(global_index: GlobalIndex) -> impl Instruction {
    GlobalInstruction {
        opcode: Opcode::GlobalGet,
        global_index,
    }
}

pub fn global_set(global_index: GlobalIndex) -> impl Instruction {
    GlobalInstruction {
        opcode: Opcode::GlobalSet,
        global_index,
    }
}

pub fn i32_load(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Load,
        memory_argument,
    }
}

pub fn i64_load(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load,
        memory_argument,
    }
}

pub fn f32_load(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::F32Load,
        memory_argument,
    }
}

pub fn f64_load(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::F64Load,
        memory_argument,
    }
}

pub fn i32_load_8_s(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Load8S,
        memory_argument,
    }
}

pub fn i32_load_8_u(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Load8U,
        memory_argument,
    }
}

pub fn i32_load_16_s(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Load16S,
        memory_argument,
    }
}

pub fn i32_load_16_u(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Load16U,
        memory_argument,
    }
}

pub fn i64_load_8_s(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load8S,
        memory_argument,
    }
}

pub fn i64_load_8_u(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load8U,
        memory_argument,
    }
}

pub fn i64_load_16_s(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load16S,
        memory_argument,
    }
}

pub fn i64_load_16_u(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load16U,
        memory_argument,
    }
}

pub fn i64_load_32_s(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load32S,
        memory_argument,
    }
}

pub fn i64_load_32_u(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Load32U,
        memory_argument,
    }
}

pub fn i32_store(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Store,
        memory_argument,
    }
}

pub fn i64_store(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Store,
        memory_argument,
    }
}

pub fn f32_store(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::F32Store,
        memory_argument,
    }
}

pub fn f64_store(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::F64Store,
        memory_argument,
    }
}

pub fn i32_store_8(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Store8,
        memory_argument,
    }
}

pub fn i32_store_16(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I32Store16,
        memory_argument,
    }
}

pub fn i64_store_8(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Store8,
        memory_argument,
    }
}

pub fn i64_store_16(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Store16,
        memory_argument,
    }
}

pub fn i64_store_32(memory_argument: MemoryArgument) -> impl Instruction {
    MemoryInstruction {
        opcode: Opcode::I64Store32,
        memory_argument,
    }
}

pub fn memory_size() -> impl Instruction {
    MemoryIndexInstruction {
        opcode: Opcode::MemorySize,
    }
}

pub fn memory_grow() -> impl Instruction {
    MemoryIndexInstruction {
        opcode: Opcode::MemoryGrow,
    }
}

pub fn i32_const(number: i32) -> impl Instruction {
    NumericInstruction {
        opcode: Opcode::I32Const,
        numeric_argument: NumericArgument::I32(number),
    }
}

pub fn i64_const(number: i64) -> impl Instruction {
    NumericInstruction {
        opcode: Opcode::I64Const,
        numeric_argument: NumericArgument::I64(number),
    }
}

pub fn f32_const(number: f32) -> impl Instruction {
    NumericInstruction {
        opcode: Opcode::F32Const,
        numeric_argument: NumericArgument::F32(number),
    }
}

pub fn f64_const(number: f64) -> impl Instruction {
    NumericInstruction {
        opcode: Opcode::F64Const,
        numeric_argument: NumericArgument::F64(number),
    }
}

pub fn i32_eqz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Eqz,
    }
}

pub fn i32_eq() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Eq,
    }
}

pub fn i32_ne() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Ne,
    }
}

pub fn i32_lt_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32LtS,
    }
}

pub fn i32_lt_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32LtU,
    }
}

pub fn i32_gt_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32GtS,
    }
}

pub fn i32_gt_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32GtU,
    }
}

pub fn i32_le_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32LeS,
    }
}

pub fn i32_le_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32LeU,
    }
}

pub fn i32_ge_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32GeS,
    }
}

pub fn i32_ge_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32GeU,
    }
}

pub fn i64_eqz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Eqz,
    }
}

pub fn i64_eq() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Eq,
    }
}

pub fn i64_ne() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Ne,
    }
}

pub fn i64_lt_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64LtS,
    }
}

pub fn i64_lt_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64LtU,
    }
}

pub fn i64_gt_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64GtS,
    }
}

pub fn i64_gt_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64GtU,
    }
}

pub fn i64_le_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64LeS,
    }
}

pub fn i64_le_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64LeU,
    }
}

pub fn i64_ge_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64GeS,
    }
}

pub fn i64_ge_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64GeU,
    }
}

pub fn f32_eq() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Eq,
    }
}

pub fn f32_ne() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Ne,
    }
}

pub fn f32_lt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Lt,
    }
}

pub fn f32_gt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Gt,
    }
}

pub fn f32_le() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Le,
    }
}

pub fn f32_ge() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Ge,
    }
}

pub fn f64_eq() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Eq,
    }
}

pub fn f64_ne() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Ne,
    }
}

pub fn f64_lt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Lt,
    }
}

pub fn f64_gt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Gt,
    }
}

pub fn f64_le() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Le,
    }
}

pub fn f64_ge() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Ge,
    }
}

pub fn i32_clz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Clz,
    }
}

pub fn i32_ctz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Ctz,
    }
}

pub fn i32_popcnt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Popcnt,
    }
}

pub fn i32_add() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Add,
    }
}

pub fn i32_sub() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Sub,
    }
}

pub fn i32_mul() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Mul,
    }
}

pub fn i32_div_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32DivS,
    }
}

pub fn i32_div_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32DivU,
    }
}

pub fn i32_rem_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32RemS,
    }
}

pub fn i32_rem_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32RemU,
    }
}

pub fn i32_and() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32And,
    }
}

pub fn i32_or() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Or,
    }
}

pub fn i32_xor() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Xor,
    }
}

pub fn i32_shl() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Shl,
    }
}

pub fn i32_shr_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32ShrS,
    }
}

pub fn i32_shr_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32ShrU,
    }
}

pub fn i32_rotl() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Rotl,
    }
}

pub fn i32_rotr() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32Rotr,
    }
}

pub fn i64_clz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Clz,
    }
}

pub fn i64_ctz() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Ctz,
    }
}

pub fn i64_popcnt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Popcnt,
    }
}

pub fn i64_add() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Add,
    }
}

pub fn i64_sub() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Sub,
    }
}

pub fn i64_mul() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Mul,
    }
}

pub fn i64_div_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64DivS,
    }
}

pub fn i64_div_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64DivU,
    }
}

pub fn i64_rem_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64RemS,
    }
}

pub fn i64_rem_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64RemU,
    }
}

pub fn i64_and() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64And,
    }
}

pub fn i64_or() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Or,
    }
}

pub fn i64_xor() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Xor,
    }
}

pub fn i64_shl() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Shl,
    }
}

pub fn i64_shr_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64ShrS,
    }
}

pub fn i64_shr_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64ShrU,
    }
}

pub fn i64_rotl() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Rotl,
    }
}

pub fn i64_rotr() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64Rotr,
    }
}

pub fn f32_abs() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Abs,
    }
}

pub fn f32_neg() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Neg,
    }
}

pub fn f32_ceil() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Ceil,
    }
}

pub fn f32_floor() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Floor,
    }
}

pub fn f32_trunc() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Trunc,
    }
}

pub fn f32_nearest() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Nearest,
    }
}

pub fn f32_sqrt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Sqrt,
    }
}

pub fn f32_add() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Add,
    }
}

pub fn f32_sub() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Sub,
    }
}

pub fn f32_mul() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Mul,
    }
}

pub fn f32_div() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Div,
    }
}

pub fn f32_min() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Min,
    }
}

pub fn f32_max() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Max,
    }
}

pub fn f32_copysign() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32Copysign,
    }
}

pub fn f64_abs() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Abs,
    }
}

pub fn f64_neg() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Neg,
    }
}

pub fn f64_ceil() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Ceil,
    }
}

pub fn f64_floor() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Floor,
    }
}

pub fn f64_trunc() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Trunc,
    }
}

pub fn f64_nearest() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Nearest,
    }
}

pub fn f64_sqrt() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Sqrt,
    }
}

pub fn f64_add() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Add,
    }
}

pub fn f64_sub() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Sub,
    }
}

pub fn f64_mul() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Mul,
    }
}

pub fn f64_div() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Div,
    }
}

pub fn f64_min() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Min,
    }
}

pub fn f64_max() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Max,
    }
}

pub fn f64_copysign() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64Copysign,
    }
}

pub fn i32_wrap_i64() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32WrapI64,
    }
}

pub fn i32_trunc_f32_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32TruncF32S,
    }
}

pub fn i32_trunc_f32_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32TruncF32U,
    }
}

pub fn i32_trunc_f64_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32TruncF64S,
    }
}

pub fn i32_trunc_f64_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32TruncF64U,
    }
}

pub fn i64_extend_i32_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64ExtendI32S,
    }
}

pub fn i64_extend_i32_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64ExtendI32U,
    }
}

pub fn i64_trunc_f32_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64TruncF32S,
    }
}

pub fn i64_trunc_f32_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64TruncF32U,
    }
}

pub fn i64_trunc_f64_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64TruncF64S,
    }
}

pub fn i64_trunc_f64_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64TruncF64U,
    }
}

pub fn f32_convert_i32_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32ConvertI32S,
    }
}

pub fn f32_convert_i32_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32ConvertI32U,
    }
}

pub fn f32_convert_i64_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32ConvertI64S,
    }
}

pub fn f32_convert_i64_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32ConvertI64U,
    }
}

pub fn f32_demote_f64() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32DemoteF64,
    }
}

pub fn f64_convert_i32_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64ConvertI32S,
    }
}

pub fn f64_convert_i32_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64ConvertI32S,
    }
}

pub fn f64_convert_i64_s() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64ConvertI64S,
    }
}

pub fn f64_convert_i64_u() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64ConvertI64U,
    }
}

pub fn f64_promote_f32() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64PromoteF32,
    }
}

pub fn i32_reinterpret_f32() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I32ReinterpretF32,
    }
}

pub fn i64_reinterpret_f64() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::I64ReinterpretF64,
    }
}

pub fn f32_reinterpret_i32() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F32ReinterpretI32,
    }
}

pub fn f64_reinterpret_i64() -> impl Instruction {
    ParameterlessInstruction {
        opcode: Opcode::F64ReinterpretI64,
    }
}

pub trait Instruction {}

pub struct ParameterlessInstruction {
    opcode: Opcode,
}

impl Instruction for ParameterlessInstruction {}

pub struct BlockInstruction {
    opcode: Opcode,
    block_type: BlockType,
}

impl Instruction for BlockInstruction {}

pub struct LabelInstruction {
    opcode: Opcode,
    label_index: LabelIndex,
}

impl Instruction for LabelInstruction {}

pub struct IndirectLabelInstruction {
    opcode: Opcode,
    labels_vector: Vec<LabelIndex>,
    label_index: LabelIndex,
}

impl Instruction for IndirectLabelInstruction {}

pub struct FunctionInstruction {
    opcode: Opcode,
    function_index: FunctionIndex,
}

impl Instruction for FunctionInstruction {}

pub struct FunctionIndirectInstruction {
    opcode: Opcode,
    type_index: TypeIndex,
}

impl Instruction for FunctionIndirectInstruction {}

pub struct LocalInstruction {
    opcode: Opcode,
    local_index: LocalIndex,
}

impl Instruction for LocalInstruction {}

pub struct GlobalInstruction {
    opcode: Opcode,
    global_index: GlobalIndex,
}

impl Instruction for GlobalInstruction {}

pub struct MemoryInstruction {
    opcode: Opcode,
    memory_argument: MemoryArgument,
}

impl Instruction for MemoryInstruction {}

pub struct MemoryIndexInstruction {
    opcode: Opcode,
}

impl Instruction for MemoryIndexInstruction {}

pub struct NumericInstruction {
    opcode: Opcode,
    numeric_argument: NumericArgument,
}

impl Instruction for NumericInstruction {}

// TODO: make sure it is represented correctly (i.e. check this TypeIndex thing in block type)
pub enum BlockType {
    EmptyType,
    ValueType(ValueType),
    TypeIndex(i32),
}

pub enum ValueType {}

pub struct LabelIndex(u32);

pub struct FunctionIndex(u32);

pub struct TypeIndex(u32);

pub struct LocalIndex(u32);

pub struct GlobalIndex(u32);

pub struct MemoryArgument {
    align: u32,
    offset: u32,
}

pub enum NumericArgument {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
}
