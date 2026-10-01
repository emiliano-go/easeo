use crate::entity::EntityType;

/// Maps an entity type to its Open Graph object type.
pub fn resolve_og_type(entity_type: &EntityType) -> &'static str {
    match entity_type {
        EntityType::Post | EntityType::Video => "article",
        _ => "website",
    }
}
