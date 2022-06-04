#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::collections::HashMap;

pub enum DataType {
    PrimaryKey,
    Integer,
    Float,
    Blob(Option<usize>),
    String(Option<usize>),
    ForiegnKey(String),
}

pub type ModelId = u64;

pub struct Model {
    temp_id: ModelId,
    fields: HashMap<String, DataType>,
}

pub struct ModelField {
    model: ModelId,
    field: String,
}

pub type RelationId = u64;

pub enum RelationKind {
    None,
    OneToOne {
        model_a: ModelField,
        model_b: ModelField,
    },
    OneToMany {
        model_a: ModelId,
        model_b: ModelField,
    },
}

pub struct ModelStore {
    models: HashMap<ModelId, Model>,
    paths: HashMap<String, ModelId>,
}

pub struct RelationStore {
    relations: HashMap<RelationId, RelationKind>,
    model_map: HashMap<ModelId, Vec<RelationId>>,
}
