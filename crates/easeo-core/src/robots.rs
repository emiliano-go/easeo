use crate::entity::Robots;

pub fn default_robots() -> Robots {
    Robots {
        index: true,
        follow: true,
        ..Default::default()
    }
}

pub fn search_robots() -> Robots {
    Robots {
        index: false,
        follow: true,
        ..Default::default()
    }
}
