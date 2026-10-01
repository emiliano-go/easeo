use crate::entity::Robots;

/// Returns the default robots directives for regular pages, `index,follow`.
pub fn default_robots() -> Robots {
    Robots {
        index: true,
        follow: true,
        ..Default::default()
    }
}

/// Returns the default robots directives for search pages, `noindex,follow`.
pub fn search_robots() -> Robots {
    Robots {
        index: false,
        follow: true,
        ..Default::default()
    }
}
