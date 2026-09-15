use crate::jsonld::SchemaContext;
use std::collections::BTreeMap;

type SchemaBuilder = Box<dyn Fn(&SchemaContext) -> serde_json::Value + Send + Sync>;

/// Registry of custom schema builders keyed by schema type.
///
/// **Python/JS limitation**: The Python and Node bindings expose `SchemaRegistry` as a type-name
/// tracker only — custom Rust closures cannot be registered from Python/JS since their callbacks
/// are not `Send + Sync`. Use `register()` from Rust only, or register Python/JS via the
/// pre-built schema builders. YAGNI: if Python/JS custom schemas are needed later, add a
/// `register_python`/`register_js` variant that stores a `PyObject`/`JsFunction` behind a mutex.
pub struct SchemaRegistry {
    builders: BTreeMap<String, SchemaBuilder>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self {
            builders: BTreeMap::new(),
        }
    }

    pub fn register<F>(&mut self, schema_type: &str, builder: F)
    where
        F: Fn(&SchemaContext) -> serde_json::Value + Send + Sync + 'static,
    {
        self.builders
            .insert(schema_type.to_string(), Box::new(builder));
    }

    pub fn get(
        &self,
        schema_type: &str,
    ) -> Option<&(dyn Fn(&SchemaContext) -> serde_json::Value + Send + Sync)> {
        self.builders.get(schema_type).map(|b| b.as_ref())
    }

    pub fn has(&self, schema_type: &str) -> bool {
        self.builders.contains_key(schema_type)
    }

    pub fn list_types(&self) -> Vec<String> {
        self.builders.keys().cloned().collect()
    }
}

impl Default for SchemaRegistry {
    fn default() -> Self {
        Self::new()
    }
}
