//! Helpers over the `castep-cell-io` 0.7 `CellDocument` model.
//!
//! These reproduce the 0.2.18 helper behavior that the seeding templates
//! depend on: element extraction, total spin, species parameter blocks,
//! and serialization to `.cell` text.

use castep_cell_fmt::{format::to_string_many_spaced, ToCellFile};
use castep_cell_io::{
    cell::{
        species::{
            Species, SpeciesLcaoState, SpeciesLcaoStates, SpeciesMass, SpeciesMassEntry,
            SpeciesPot, SpeciesPotEntry,
        },
        species_params::SpeciesParams,
    },
    CellDocument, Positions,
};
use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementSymbol, LookupElement},
};
use std::str::FromStr;

fn species_to_element(species: &Species) -> Option<ElementSymbol> {
    match species {
        Species::Symbol(sym) => ElementSymbol::from_str(sym).ok(),
        Species::AtomicNumber(num) => ElementSymbol::try_from(*num).ok(),
    }
}

fn position_entries(positions: &Positions) -> Vec<&Species> {
    match positions {
        Positions::Frac(frac) => frac.positions.iter().map(|p| &p.species).collect(),
        Positions::Abs(abs) => abs.positions.iter().map(|p| &p.species).collect(),
    }
}

/// Unique element symbols present in the document positions.
///
/// Sorted and deduplicated, matching the old `CellDocument::get_elements`
/// semantics.
pub fn cell_elements(doc: &CellDocument) -> Vec<ElementSymbol> {
    let mut elements = position_entries(&doc.positions)
        .iter()
        .filter_map(|s| species_to_element(s))
        .collect::<Vec<_>>();
    elements.sort();
    elements.dedup();
    elements
}

/// Total initial spin polarization of all positions.
///
/// Accumulates the spin of every position, matching the old
/// `CellDocument::total_spin` semantics.
pub fn total_spin(doc: &CellDocument) -> u32 {
    position_entries(&doc.positions)
        .iter()
        .filter_map(|s| species_to_element(s))
        .map(|elm| u32::from(ELEMENT_TABLE.get_by_symbol(elm).spin()))
        .sum()
}

/// Pseudopotential filenames for every element in the document, from the
/// periodic table.
pub fn potential_filenames(elements: &[ElementSymbol]) -> Vec<String> {
    elements
        .iter()
        .map(|&elm| ELEMENT_TABLE.get_by_symbol(elm).potential().to_string())
        .collect()
}

/// Build the species parameter blocks (mass, pot, LCAO states) for the
/// given elements from the periodic table.
///
/// Replaces the old `Species*Block::from_elements` constructors.
pub fn species_params_for(elements: &[ElementSymbol]) -> SpeciesParams {
    SpeciesParams {
        species_mass: Some(SpeciesMass {
            unit: None,
            masses: elements
                .iter()
                .map(|&elm| SpeciesMassEntry {
                    species: Species::Symbol(elm.to_string()),
                    mass: ELEMENT_TABLE.get_by_symbol(elm).mass(),
                })
                .collect(),
        }),
        species_pot: Some(SpeciesPot {
            potentials: elements
                .iter()
                .map(|&elm| SpeciesPotEntry {
                    species: Species::Symbol(elm.to_string()),
                    filename: ELEMENT_TABLE.get_by_symbol(elm).potential().to_string(),
                })
                .collect(),
        }),
        species_lcao_states: Some(SpeciesLcaoStates {
            states: elements
                .iter()
                .map(|&elm| SpeciesLcaoState {
                    species: Species::Symbol(elm.to_string()),
                    num_states: u32::from(ELEMENT_TABLE.get_by_symbol(elm).lcao()),
                })
                .collect(),
        }),
        ..Default::default()
    }
}

/// Serialize a `CellDocument` to `.cell` file text.
pub fn cell_to_string(doc: &CellDocument) -> String {
    to_string_many_spaced(&doc.to_cell_file())
}
