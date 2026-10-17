use std::collections::{BTreeMap, HashMap, HashSet};

use crate::member::Member;

pub fn generate_next_year(family: &[Member]) -> Result<BTreeMap<String, Vec<String>>, String> {
    if family.len() < 3 {
        return Err("Il faut au moins 3 membres pour effectuer le tirage.".to_string());
    }

    let mut names: Vec<String> = family.iter().map(|m| m.name.clone()).collect();

    names.sort();

    if names.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err("Les noms des membres doivent être uniques.".to_string());
    }

    // Init forbiddens
    let forbidden: HashMap<String, HashSet<String>> = family
        .iter()
        .map(|member| (member.name.clone(), member.gifts.iter().cloned().collect()))
        .collect();

    let mut received: HashMap<String, usize> = names.iter().map(|name| (name.clone(), 0)).collect();

    let mut result: BTreeMap<String, Vec<String>> = BTreeMap::new();

    if search(0, &names, &forbidden, &mut received, &mut result) {
        Ok(result)
    } else {
        Err("Aucun tirage valide ne respecte toutes les contraintes.".to_string())
    }
}

fn search(
    index: usize,
    names: &[String],
    forbidden: &HashMap<String, HashSet<String>>,
    received: &mut HashMap<String, usize>,
    result: &mut BTreeMap<String, Vec<String>>,
) -> bool {
    if index == names.len() {
        return received.values().all(|&count| count == 2);
    }

    let remaining_givers = names.len() - index;

    for name in names {
        if received[name] + remaining_givers < 2 {
            return false;
        }
    }

    let giver = &names[index];
    let already_offered = &forbidden[giver];

    let candidates: Vec<String> = names
        .iter()
        .filter(|receiver| {
            *receiver != giver && !already_offered.contains(*receiver) && received[*receiver] < 2
        })
        .cloned()
        .collect();

    for i in 0..candidates.len() {
        for j in (i + 1)..candidates.len() {
            let first = &candidates[i];
            let second = &candidates[j];

            *received.get_mut(first).unwrap() += 1;
            *received.get_mut(second).unwrap() += 1;

            result.insert(giver.clone(), vec![first.clone(), second.clone()]);

            if search(index + 1, names, forbidden, received, result) {
                return true;
            }
            
            *received.get_mut(first).unwrap() -= 1;
            *received.get_mut(second).unwrap() -= 1;
            result.remove(giver);
        }
    }

    false
}
