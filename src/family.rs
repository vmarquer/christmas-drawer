use crate::member::Member;
use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fs;
use std::path::Path;

fn handle_member(giver: String, receivers: &Vec<String>, family: &mut Vec<Member>) {
    if let Some(member) = family.iter_mut().find(|m| m.name == giver) {
        for receiver in receivers {
            if !member.gifts.contains(receiver) {
                member.gifts.push(receiver.clone());
            }
        }
    } else {
        family.push(Member {
            name: giver,
            gifts: receivers.clone(),
        });
    }
}

pub fn load_family(path: impl AsRef<Path>) -> Result<Vec<Member>, Box<dyn Error>> {
    // Read json's content
    let content = fs::read_to_string(path)?;

    // Init historic and names
    let historic: BTreeMap<String, BTreeMap<String, Vec<String>>> = serde_json::from_str(&content)?;
    let mut names = BTreeSet::new();

    // Fill names set
    for givers in historic.values() {
        for (giver, receivers) in givers {
            names.insert(giver.clone());
            for receiver in receivers {
                names.insert(receiver.clone());
            }
        }
    }

    // Init family object
    let mut family: Vec<Member> = vec![];

    for givers in historic.values() {
        for (giver, receivers) in givers {
            handle_member(giver.to_string(), receivers, &mut family);
        }
    }

    Ok(family)
}
