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
    id: ModelId,
    fields: Vec<(String, DataType)>,
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
    pub models: HashMap<ModelId, Model>,
    pub models_count: u64,
    pub paths: HashMap<String, ModelId>,
    pub relations: HashMap<RelationId, RelationKind>,
    pub relations_count: u64,
    pub model_relations: HashMap<ModelId, Vec<RelationId>>,
}

impl ModelStore {
    fn new() -> Self {
        ModelStore {
            models: HashMap::new(),
            models_count: 0,
            paths: HashMap::new(),
            relations: HashMap::new(),
            relations_count: 0,
            model_relations: HashMap::new(),
        }
    }
    fn add_model<I>(&mut self, path: String, fields_iter: I) -> ModelId
        where I: IntoIterator<Item=(String, DataType)>
    {
        // todo!();
        let mut fields = Vec::new();
        for (name, dt) in fields_iter {
            todo!();
        }
        let id = self.models_count;
        let mut model = Model {
            id,
            fields,
        };
        self.models_count = id + 1;
        self.models.insert(id, model);
        id
    }
}