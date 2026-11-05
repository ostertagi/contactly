//! JSON persistence: one file per user in ./data/

use crate::model::{Contact, Group};
use std::fs;
use std::path::PathBuf;

fn path_for(user: &str) -> PathBuf {
    PathBuf::from("data").join(format!("contacts_{user}.json"))
}

/// Loads the user's contacts. First run (no file yet) seeds sample data.
pub fn load(user: &str) -> Vec<Contact> {
    match fs::read_to_string(path_for(user)) {
        Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
        Err(_) => sample_contacts(),
    }
}

pub fn save(user: &str, contacts: &[Contact]) -> Result<(), String> {
    fs::create_dir_all("data").map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(contacts).map_err(|e| e.to_string())?;
    fs::write(path_for(user), json).map_err(|e| e.to_string())
}

fn sample_contacts() -> Vec<Contact> {
    let c = |id, name: &str, email: &str, phone: &str, group, favorite| Contact {
        id,
        name: name.into(),
        email: email.into(),
        phone: phone.into(),
        group,
        favorite,
    };
    vec![
        c(1, "Amina Benali", "amina@example.com", "+212 661 000 111", Group::Family, true),
        c(2, "Youssef Idrissi", "youssef@example.com", "+212 662 000 222", Group::Friends, false),
        c(3, "Sarah Connor", "sarah@work.example", "+1 555 0100", Group::Work, false),
        c(4, "Karim Alaoui", "karim@example.com", "+212 663 000 333", Group::Friends, true),
        c(5, "Linda Moreau", "linda@work.example", "+33 6 00 00 00 05", Group::Work, false),
    ]
}
