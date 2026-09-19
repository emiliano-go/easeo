use crate::jsonld::SchemaContext;
use std::collections::BTreeMap;

type SchemaBuilder = Box<dyn Fn(&SchemaContext) -> serde_json::Value + Send + Sync>;

/// Registry of custom schema builders keyed by schema type.
///
/// This is the Rust-only registry: builders are closures that must be
/// `Send + Sync`, so Python and JavaScript callables cannot be stored here.
///
/// Python and JavaScript get custom schema support through a registry in
/// their own layer (`easeo.registry.SchemaRegistry` and the `SchemaRegistry`
/// class in `@easeo/core`). Those registries store native callables and apply
/// the generated schema to the built payload, so registration works from all
/// three languages. The native binding types expose `has` and `list_types`
/// for introspection only.
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
