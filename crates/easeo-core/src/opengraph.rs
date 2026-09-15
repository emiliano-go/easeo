use crate::entity::EntityType;

pub fn resolve_og_type(entity_type: &EntityType) -> &'static str {
    match entity_type {
        EntityType::Post | EntityType::Video => "article",
        _ => "website",
    }
}
