mod family;
mod generator;
mod member;
mod saver;
use family::load_family;
use generator::generate_next_year;
use saver::save_year;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = "2027";
    let family = load_family("data/historic.json")
        .map_err(|e| format!("Impossible de charger la famille : {e}"))?;
    let tirage = generate_next_year(&family)
        .map_err(|e| format!("Impossible de générer le tirage : {e}"))?;
    save_year(year, &tirage, "data/historic.json")
        .map_err(|e| format!("Impossible d'enregistrer le tirage : {e}"))?;
    Ok(())
}
