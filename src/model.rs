//! Plain data types. Nothing in here knows about Iced.

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Group {
    Family,
    Friends,
    Work,
}

impl Group {
    pub const ALL: &'static [Group] = &[Group::Family, Group::Friends, Group::Work];
}

// pick_list needs Display to render each option.
impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Group::Family => "Family",
            Group::Friends => "Friends",
            Group::Work => "Work",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub group: Group,
    pub favorite: bool,
}

impl Contact {
    /// The search + filter logic lives with the data, so it can be unit-tested
    /// without any GUI.
    pub fn matches(&self, query: &str, group: Option<Group>, favorites_only: bool) -> bool {
        if let Some(g) = group {
            if self.group != g {
                return false;
            }
        }
        if favorites_only && !self.favorite {
            return false;
        }
        let q = query.trim().to_lowercase();
        q.is_empty()
            || self.name.to_lowercase().contains(&q)
            || self.email.to_lowercase().contains(&q)
            || self.phone.contains(&q)
    }
}

/// Who is logged in. Its presence in `App` *is* the "authenticated" state.
#[derive(Debug, Clone)]
pub struct Session {
    pub username: String,
    pub display_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Contact {
        Contact {
            id: 1,
            name: "Alice Martin".into(),
            email: "alice@example.com".into(),
            phone: "+212 600 000 001".into(),
            group: Group::Work,
            favorite: false,
        }
    }

    #[test]
    fn search_is_case_insensitive() {
        assert!(sample().matches("ALICE", None, false));
        assert!(sample().matches("example.com", None, false));
        assert!(!sample().matches("bob", None, false));
    }

    #[test]
    fn filters_combine() {
        assert!(sample().matches("", Some(Group::Work), false));
        assert!(!sample().matches("", Some(Group::Family), false));
        assert!(!sample().matches("", None, true));
    }
}
