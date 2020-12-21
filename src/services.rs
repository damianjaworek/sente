use super::wasm::{
    self,
    indices::{FunctionId, LocalId, TypeId},
};

#[derive(Clone)]
pub struct NameService {
    named_entities: Vec<NamedEntity>,
    next_function_id: u32,
    pub next_local_id: u32,
    outer: Option<Box<NameService>>,
}

impl Default for NameService {
    fn default() -> Self {
        NameService {
            named_entities: Vec::new(),
            next_function_id: 0,
            next_local_id: 0,
            outer: None,
        }
    }
}

impl NameService {
    pub fn enclosed(enclosing: NameService) -> NameService {
        NameService {
            next_local_id: enclosing.next_local_id,
            outer: Some(Box::new(enclosing)),
            ..Default::default()
        }
    }

    pub fn define_function(
        &mut self,
        function_name: String,
        type_id: TypeId,
    ) -> Result<(), String> {
        if self.is_defined(&function_name) {
            Err(format!("Name `{}` is already defined", function_name))
        } else {
            let function_id = FunctionId::new(self.next_function_id);
            let function = NamedEntity::new_function(function_name, function_id, type_id);
            self.next_function_id += 1;
            self.named_entities.push(function);
            Ok(())
        }
    }

    pub fn define_local_variable(
        &mut self,
        variable_name: String,
        variable_type: PrimitiveType,
    ) -> Result<(), String> {
        if self.is_defined(&variable_name) {
            Err(format!("Name `{}` is already defined", variable_name))
        } else {
            let local_id = LocalId::new(self.next_local_id);
            let variable = NamedEntity::new_local_variable(variable_name, local_id, variable_type);
            self.next_local_id += 1;
            self.named_entities.push(variable);
            Ok(())
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<NamedEntity> {
        self.named_entities
            .clone()
            .into_iter()
            .find(|entity| entity.is_named(name))
            .or_else(|| self.outer.clone().and_then(|o| o.find_by_name(name)))
    }

    pub fn find_local_variable_by_name(&self, name: &str) -> Option<NamedLocalVariable> {
        if let Some(NamedEntity::LocalVariable(variable)) = self.find_by_name(name) {
            Some(variable)
        } else {
            None
        }
    }

    pub fn find_function_by_name(&self, name: &str) -> Option<NamedFunction> {
        if let Some(NamedEntity::Function(function)) = self.find_by_name(name) {
            Some(function)
        } else {
            None
        }
    }

    pub fn emit_function_section(&self) -> wasm::sections::Function {
        let functions = self
            .named_entities
            .iter()
            .filter_map(|f| f.map_to_function())
            .map(|f| f.type_id)
            .collect();

        wasm::sections::Function::new(functions)
    }

    pub fn emit_export_section(&self) -> wasm::sections::Export {
        let entries = self
            .named_entities
            .iter()
            .filter_map(|f| f.map_to_function())
            .map(|f| f.to_export_entry())
            .collect();

        wasm::sections::Export::new(entries)
    }

    pub fn get_locals(&self) -> Vec<wasm::sections::Locals> {
        self.named_entities
            .iter()
            .filter_map(|e| e.map_to_local_variable())
            .map(|v| wasm::sections::Locals::new(1, v.get_variable_type().into()))
            .collect()
    }

    fn is_defined(&self, name: &str) -> bool {
        self.named_entities
            .iter()
            .any(|entity| entity.is_named(name))
    }
}

#[derive(Clone)]
pub struct TypeService {
    defined_types: Vec<Type>,
    next_type_id: u32,
}

impl Default for TypeService {
    fn default() -> Self {
        let defined_types = vec![
            Type::new_primitive_type(PrimitiveType::Int32),
            Type::new_primitive_type(PrimitiveType::Int64),
            Type::new_primitive_type(PrimitiveType::Float32),
            Type::new_primitive_type(PrimitiveType::Float64),
        ];
        let next_type_id = 0;

        TypeService {
            defined_types,
            next_type_id,
        }
    }
}

impl TypeService {
    pub fn define_function_type(
        &mut self,
        parameter_types: Vec<PrimitiveType>,
        return_type: PrimitiveType,
    ) -> TypeId {
        let type_id = TypeId::new(self.next_type_id);
        let function_type = Type::new_function_type(parameter_types, return_type, type_id);

        if !self.defined_types.contains(&function_type) {
            self.defined_types.push(function_type);
            self.next_type_id += 1;
            type_id
        } else {
            self.defined_types
                .iter()
                .filter_map(|t| t.map_to_function_type())
                .find(|t| {
                    t == &function_type
                        .map_to_function_type()
                        .expect("Cannot map to function type")
                })
                .expect("Cannot find defined function type")
                .type_id
        }
    }

    pub fn emit_type_section(&self) -> wasm::sections::Type {
        let function_types: Vec<wasm::types::FunctionType> = self
            .defined_types
            .iter()
            .filter_map(|t| t.map_to_function_type())
            .map(|t| t.into())
            .collect();

        wasm::sections::Type::new(function_types)
    }

    pub fn find_type_by_id(&self, type_id: TypeId) -> Option<FunctionType> {
        self.defined_types
            .iter()
            .find(|t| t.has_type_id_equal(type_id))
            .map(|t| {
                t.map_to_function_type()
                    .expect("Cannot map to function type")
            })
    }
}

#[derive(Clone, Debug)]
pub enum NamedEntity {
    LocalVariable(NamedLocalVariable),
    Function(NamedFunction),
}

impl NamedEntity {
    fn new_local_variable(
        name: String,
        local_id: LocalId,
        variable_type: PrimitiveType,
    ) -> NamedEntity {
        NamedEntity::LocalVariable(NamedLocalVariable::new(name, local_id, variable_type))
    }

    fn new_function(name: String, function_id: FunctionId, type_id: TypeId) -> NamedEntity {
        NamedEntity::Function(NamedFunction::new(name, function_id, type_id))
    }

    fn is_named(&self, name: &str) -> bool {
        match self {
            NamedEntity::LocalVariable(local) => local.name == name,
            NamedEntity::Function(function) => function.name == name,
        }
    }

    fn _name(&self) -> String {
        match self {
            NamedEntity::LocalVariable(local) => local.name.to_owned(),
            NamedEntity::Function(function) => function.name.to_owned(),
        }
    }

    fn map_to_function(&self) -> Option<NamedFunction> {
        match self {
            NamedEntity::Function(function) => Some(function.clone()),
            _ => None,
        }
    }

    fn map_to_local_variable(&self) -> Option<NamedLocalVariable> {
        match self {
            NamedEntity::LocalVariable(variable) => Some(variable.clone()),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct NamedLocalVariable {
    name: String,
    local_id: LocalId,
    variable_type: PrimitiveType,
}

impl NamedLocalVariable {
    fn new(name: String, local_id: LocalId, variable_type: PrimitiveType) -> NamedLocalVariable {
        NamedLocalVariable {
            name,
            local_id,
            variable_type,
        }
    }

    pub fn get_local_id(&self) -> LocalId {
        self.local_id
    }

    pub fn get_variable_type(&self) -> PrimitiveType {
        self.variable_type.clone()
    }
}

#[derive(Clone, Debug)]
pub struct NamedFunction {
    name: String,
    function_id: FunctionId,
    type_id: TypeId,
}

impl NamedFunction {
    fn new(name: String, function_id: FunctionId, type_id: TypeId) -> NamedFunction {
        NamedFunction {
            name,
            function_id,
            type_id,
        }
    }

    pub fn get_function_id(&self) -> FunctionId {
        self.function_id
    }

    pub fn get_type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn to_export_entry(&self) -> wasm::sections::ExportEntry {
        let name = wasm::sections::Name::new(self.name.clone());
        let export_description = wasm::sections::ExportDescription::Function(self.function_id);
        wasm::sections::ExportEntry::new(name, export_description)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimitiveType),
    Function(FunctionType),
}

impl Type {
    fn new_primitive_type(primitive_type: PrimitiveType) -> Type {
        Type::Primitive(primitive_type)
    }

    fn new_function_type(
        parameter_types: Vec<PrimitiveType>,
        return_type: PrimitiveType,
        type_id: TypeId,
    ) -> Type {
        Type::Function(FunctionType::new(parameter_types, return_type, type_id))
    }

    fn map_to_function_type(&self) -> Option<FunctionType> {
        match self {
            Type::Function(function) => Some(function.clone()),
            _ => None,
        }
    }

    fn has_type_id_equal(&self, type_id: TypeId) -> bool {
        matches!(self, Type::Function(function) if function.type_id == type_id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PrimitiveType {
    Int32,
    Int64,
    Float32,
    Float64,
}

impl Into<wasm::types::ValueType> for PrimitiveType {
    fn into(self) -> wasm::types::ValueType {
        match self {
            PrimitiveType::Int32 => wasm::types::ValueType::I32,
            PrimitiveType::Int64 => wasm::types::ValueType::I64,
            PrimitiveType::Float32 => wasm::types::ValueType::F32,
            PrimitiveType::Float64 => wasm::types::ValueType::F64,
        }
    }
}

impl Into<super::ast::ExpressionType> for PrimitiveType {
    fn into(self) -> super::ast::ExpressionType {
        match self {
            PrimitiveType::Int32 => super::ast::ExpressionType::Int32,
            PrimitiveType::Int64 => super::ast::ExpressionType::Int64,
            PrimitiveType::Float32 => super::ast::ExpressionType::Float32,
            PrimitiveType::Float64 => super::ast::ExpressionType::Float64,
        }
    }
}

impl From<super::ast::Type> for PrimitiveType {
    fn from(ast_type: super::ast::Type) -> Self {
        match ast_type {
            super::ast::Type::Int32 => PrimitiveType::Int32,
            super::ast::Type::Int64 => PrimitiveType::Int64,
            super::ast::Type::Float32 => PrimitiveType::Float32,
            super::ast::Type::Float64 => PrimitiveType::Float64,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FunctionType {
    parameter_types: Vec<PrimitiveType>,
    return_type: PrimitiveType,
    type_id: TypeId,
}

impl FunctionType {
    fn new(
        parameter_types: Vec<PrimitiveType>,
        return_type: PrimitiveType,
        type_id: TypeId,
    ) -> FunctionType {
        FunctionType {
            parameter_types,
            return_type,
            type_id,
        }
    }

    pub fn get_parameter_types(&self) -> Vec<PrimitiveType> {
        self.parameter_types.clone()
    }

    pub fn _get_type_id(&self) -> TypeId {
        self.type_id
    }

    pub fn get_return_type(&self) -> PrimitiveType {
        self.return_type.clone()
    }
}

impl PartialEq for FunctionType {
    fn eq(&self, other: &Self) -> bool {
        self.parameter_types == other.parameter_types && self.return_type == other.return_type
    }
}

impl Eq for FunctionType {}

impl Into<wasm::types::FunctionType> for FunctionType {
    fn into(self) -> wasm::types::FunctionType {
        let parameters = wasm::types::ResultType::new(
            self.parameter_types.into_iter().map(|t| t.into()).collect(),
        );
        let results = wasm::types::ResultType::new(vec![self.return_type.into()]);

        wasm::types::FunctionType::new(parameters, results)
    }
}
