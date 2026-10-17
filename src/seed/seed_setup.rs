use castep_cell_io::{
    cell::{
        bz_sampling_kpoints::{BsKpointPathSpacing, KpointsMpSpacing},
        constraints::{FixAllCell, FixCOM, IonicConstraints},
        constraints_params::ConstraintsParams,
        external_field_params::ExternalFieldParams,
        kpoints_params::KpointsParams,
        species_params::SpeciesParams,
        spectral_params::SpectralParams,
    },
    units::InvLengthUnit,
    CellDocument,
};
use castep_periodic_table::{
    data::ELEMENT_TABLE,
    element::{ElementFamily, LookupElement},
};
use std::path::Path;

use crate::{
    cell_doc::{cell_elements, species_params_for, total_spin},
    cutoff::{EnergyCutoff, EnergyCutoffError},
    error::SeedingErrors,
    param::{CastepParam, MetalsMethod, NumOccCycles},
    CastepTask,
};

fn geom_opt_entries(template_cell: &CellDocument) -> (ConstraintsParams, SpeciesParams) {
    let elements = cell_elements(template_cell);
    let constraints = ConstraintsParams {
        fix_all_cell: Some(FixAllCell(true)),
        fix_com: Some(FixCOM(false)),
        ionic_constraints: Some(IonicConstraints {
            constraints: Vec::new(),
        }),
        ..Default::default()
    };
    (constraints, species_params_for(&elements))
}

pub trait CellBuilding {
    fn geom_opt_cell_template(template_cell: &CellDocument) -> CellDocument {
        let (constraints, species) = geom_opt_entries(template_cell);
        let mut geom_cell = template_cell.clone();
        geom_cell.kpoints = KpointsParams {
            kpoints_mp_spacing: Some(KpointsMpSpacing {
                value: 0.07,
                unit: Some(InvLengthUnit::Angstrom),
            }),
            ..Default::default()
        };
        geom_cell.constraints = constraints;
        geom_cell.external_fields = ExternalFieldParams::default();
        geom_cell.species = species;
        geom_cell
    }

    #[cfg(feature = "template")]
    fn bs_cell_template(template_cell: &CellDocument) -> CellDocument {
        let mut bs_cell = Self::geom_opt_cell_template(template_cell);
        bs_cell.spectral = SpectralParams {
            bs_kpoint_path_spacing: Some(BsKpointPathSpacing {
                value: 0.07,
                unit: Some(InvLengthUnit::Angstrom),
            }),
            ..Default::default()
        };
        bs_cell
    }

    fn build_cell_for_task(
        &self,
        template_cell: &CellDocument,
        castep_task: CastepTask,
    ) -> CellDocument {
        #[cfg(feature = "template")]
        match castep_task {
            CastepTask::BandStructure => Self::bs_cell_template(template_cell),
            CastepTask::GeometryOptimization => Self::geom_opt_cell_template(template_cell),
            _ => template_cell.clone(),
        }
        #[cfg(not(feature = "template"))]
        {
            let _ = castep_task;
            template_cell.clone()
        }
    }
}

pub trait ParamBuilding {
    fn cutoff_energy<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        potentials_loc: P,
    ) -> Result<f64, EnergyCutoffError> {
        let cutoff_energies = cell_elements(template_cell)
            .iter()
            .map(|&elm| {
                let potential_name = ELEMENT_TABLE.get_by_symbol(elm).potential();
                let potential_file = Path::new(potentials_loc.as_ref()).join(potential_name);
                energy_cutoff.get_cutoff_energy_from_pot(potential_file)
            })
            .collect::<Result<Vec<f64>, EnergyCutoffError>>()?;
        Ok(cutoff_energies
            .into_iter()
            .reduce(|prev, next| if next > prev { next } else { prev })
            .expect("Error in comparing the largest cutoff energy"))
    }
    fn build_param_for_task(
        &self,
        template_cell: &CellDocument,
        castep_task: CastepTask,
    ) -> Result<CastepParam, SeedingErrors>;

    #[cfg(feature = "template")]
    fn geom_opt_param_template<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        use_edft: bool,
        potentials_loc: P,
    ) -> Result<CastepParam, SeedingErrors> {
        let use_edft = if use_edft {
            cell_elements(template_cell).iter().any(|elm| {
                matches!(
                    elm.family(),
                    ElementFamily::RareEarthLa | ElementFamily::RareEarthAc
                )
            })
        } else {
            false
        };
        let mut param = CastepParam::geom_opt_param_template(
            self.cutoff_energy(template_cell, energy_cutoff, potentials_loc)?,
            f64::from(total_spin(template_cell)),
        )
        .map_err(SeedingErrors::ParamBuildingError)?;
        if use_edft {
            if let Some(electro_min) = param.electro_min.as_mut() {
                electro_min.metals_method = Some(MetalsMethod::EDFT);
                electro_min.num_occ_cycles = Some(NumOccCycles::from(6));
            }
        }
        Ok(param)
    }

    #[cfg(feature = "template")]
    fn dos_param_template<P: AsRef<Path>>(
        &self,
        template_cell: &CellDocument,
        energy_cutoff: EnergyCutoff,
        use_edft: bool,
        potentials_loc: P,
    ) -> Result<CastepParam, SeedingErrors> {
        let use_edft = if use_edft {
            cell_elements(template_cell).iter().any(|elm| {
                matches!(
                    elm.family(),
                    ElementFamily::RareEarthLa | ElementFamily::RareEarthAc
                )
            })
        } else {
            false
        };
        let mut param = CastepParam::dos_opt_param_template(
            self.cutoff_energy(template_cell, energy_cutoff, potentials_loc)?,
            f64::from(total_spin(template_cell)),
        )
        .map_err(SeedingErrors::ParamBuildingError)?;
        if use_edft {
            if let Some(electro_min) = param.electro_min.as_mut() {
                electro_min.metals_method = Some(MetalsMethod::EDFT);
                electro_min.num_occ_cycles = Some(NumOccCycles::from(6));
            }
        }
        Ok(param)
    }
}
