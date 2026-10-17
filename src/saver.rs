
use std::collections::BTreeMap;
use std::error::Error;
use std::fs;
use std::path::Path;

use serde_json::Value;

pub fn save_year(
    year: &str,
    tirage: &BTreeMap<String, Vec<String>>,
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    // Lire le fichier JSON existant.
    let content = fs::read_to_string(&path)?;

    // Parser le JSON.
    let mut historique: Value = serde_json::from_str(&content)?;

    // Vérifier que la racine du JSON est un objet.
    let annees = historique
        .as_object_mut()
        .ok_or("Le JSON doit contenir un objet à la racine")?;

    // Ne pas écraser accidentellement une année existante.
    if annees.contains_key(year) {
        return Err(format!("L'année {year} existe déjà dans l'historique").into());
    }

    // Convertir le tirage en valeur JSON.
    let nouveau_tirage = serde_json::to_value(tirage)?;

    // Ajouter la nouvelle année.
    annees.insert(year.to_string(), nouveau_tirage);

    // Sérialiser le JSON avec une indentation lisible.
    let json = serde_json::to_string_pretty(&historique)?;

    // Enregistrer le fichier.
    fs::write(path, format!("{json}\n"))?;

    println!("Tirage de {year} enregistré avec succès.");

    Ok(())
}
