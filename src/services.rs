use super::wasm::indices::{FunctionId, LocalId, TypeId};
use std::collections::HashMap;

pub struct NameService {
    named_entities: Vec<NamedEntity>,
}

impl Default for NameService {
    fn default() -> Self {
        NameService {
            named_entities: Vec::new(),
        }
    }
}

impl NameService {
    pub fn define(&mut self, new_entity: NamedEntity) -> Result<(), String> {
        if self
            .named_entities
            .iter()
            .any(|entity| entity.is_named(&new_entity.name()))
        {
            Err(format!("Name `{}` is already defined", new_entity.name()))
        } else {
            self.named_entities.push(new_entity);
            Ok(())
        }
    }

    pub fn find_by_name(&self, name: &str) -> Option<&NamedEntity> {
        self.named_entities
            .iter()
            .find(|entity| entity.is_named(name))
    }
}

pub struct TypeService {
    defined_types: Vec<Type>,
    type_ids: HashMap<FunctionType, TypeId>,
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
        let type_ids = HashMap::new();
        let next_type_id = 0;

        TypeService {
            defined_types,
            type_ids,
            next_type_id,
        }
    }
}

impl TypeService {
    pub fn define(&mut self, new_type: Type) {
        if !self.defined_types.contains(&new_type) {
            if let Type::Function(function) = &new_type {
                self.type_ids
                    .insert(function.clone(), TypeId::new(self.next_type_id));
                self.next_type_id += 1;
            }
            self.defined_types.push(new_type);
        }
    }

    pub fn get_type_id(&self, function_type: &FunctionType) -> Option<&TypeId> {
        self.type_ids.get(function_type)
    }
}

#[derive(Clone, Debug)]
pub enum NamedEntity {
    LocalVariable(NamedLocalVariable),
    Function(NamedFunction),
}

impl NamedEntity {
    pub fn new_local_variable(name: String, local_id: LocalId) -> NamedEntity {
        NamedEntity::LocalVariable(NamedLocalVariable::new(name, local_id))
    }

    pub fn new_function(name: String, function_id: FunctionId) -> NamedEntity {
        NamedEntity::Function(NamedFunction::new(name, function_id))
    }

    pub fn is_named(&self, name: &str) -> bool {
        match self {
            NamedEntity::LocalVariable(local) => local.name == name,
            NamedEntity::Function(function) => function.name == name,
        }
    }

    pub fn name(&self) -> String {
        match self {
            NamedEntity::LocalVariable(local) => local.name.to_owned(),
            NamedEntity::Function(function) => function.name.to_owned(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct NamedLocalVariable {
    name: String,
    local_id: LocalId,
}

impl NamedLocalVariable {
    pub fn new(name: String, local_id: LocalId) -> NamedLocalVariable {
        NamedLocalVariable { name, local_id }
    }

    pub fn get_local_id(&self) -> LocalId {
        self.local_id
    }
}

#[derive(Clone, Debug)]
pub struct NamedFunction {
    name: String,
    function_id: FunctionId,
}

impl NamedFunction {
    pub fn new(name: String, function_id: FunctionId) -> NamedFunction {
        NamedFunction { name, function_id }
    }

    pub fn get_function_id(&self) -> FunctionId {
        self.function_id
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    Primitive(PrimitiveType),
    Function(FunctionType),
}

impl Type {
    pub fn new_primitive_type(primitive_type: PrimitiveType) -> Type {
        Type::Primitive(primitive_type)
    }

    pub fn new_function_type(
        parameter_types: Vec<PrimitiveType>,
        return_type: PrimitiveType,
    ) -> Type {
        Type::Function(FunctionType::new(parameter_types, return_type))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Int32,
    Int64,
    Float32,
    Float64,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionType {
    parameter_types: Vec<PrimitiveType>,
    return_type: PrimitiveType,
}

impl FunctionType {
    pub fn new(parameter_types: Vec<PrimitiveType>, return_type: PrimitiveType) -> FunctionType {
        FunctionType {
            parameter_types,
            return_type,
        }
    }
}
