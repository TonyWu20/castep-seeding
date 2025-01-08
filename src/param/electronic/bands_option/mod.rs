use std::fmt::Display;

use derive_builder::Builder;
pub use extra_bands::ExtraBands;
use nbands::Nbands;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

mod extra_bands;
/// This keyword determines the maximum number of bands at any k-point and spin.
/// There are three ways in which you can specify the maximum number of bands at any k-point and spin:
/// Directly, using `NBANDS`.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands using `NEXTRA_BANDS`.
/// This is the method used in the CASTEP interface.
/// Indirectly, by specifying the number of extra bands in addition to the number of occupied bands as a percentage of the latter value using `PERC_EXTRA_BANDS`.
/// It is not possible to have both the `NBANDS` keyword and either the `NEXTRA_BANDS` or `PERC_EXTRA_BANDS` keywords present in the same input file.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize, Builder, Default)]
#[builder(setter(into, strip_option), default)]
pub struct BandsOption {
    nbands: Option<Nbands>,
    extra_bands: Option<ExtraBands>,
}

impl Display for BandsOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.nbands.map(|v| v.output()),
            self.extra_bands.map(|v| v.output()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{output}")
    }
}

mod nbands {
    use std::fmt::Display;

    use castep_seeding_derive::KeywordDisplay;
    use serde::{Deserialize, Serialize};

    use crate::param::{electronic::spin_polarised::SpinPolarised, KeywordDisplay};

    use super::extra_bands::ExtraBands;

    #[derive(
        Debug,
        Clone,
        Copy,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Hash,
        Serialize,
        Deserialize,
        KeywordDisplay,
    )]
    #[keyword_display(field = "NBANDS", from = u64)]
    pub struct Nbands(u64);

    impl Display for Nbands {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl Nbands {
        /// If NEXTRA_BANDS is specified and SPIN_POLARIZED : FALSE:
        /// NBANDS : (NELECTRONS/2) + NEXTRA_BANDS
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN) + NEXTRA_BANDS.
        /// If PERC_EXTRA_BANDS is specified and SPIN_POLARIZED : FALSE:
        ///     NBANDS : (NELECTRONS/2) × [ 1 + (PERC_EXTRA_BANDS/100)]
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN) × [ 1 + (PERC_EXTRA_BANDS/100)]
        /// If NBANDS, NEXTRA_BANDS and PERC_EXTRA_BANDS are not specified and FIX_OCCUPANCY : TRUE, then, if SPIN_POLARIZED : FALSE:
        ///     NBANDS : NELECTRONS/2.
        /// Or, if SPIN_POLARIZED : TRUE:
        ///     NBANDS : max(NUP, NDOWN)
        /// If FIX_OCCUPANCY : FALSE, these default values of NBANDS will be multiplied by 1.2.
        pub fn default_value(
            spin_polarised: SpinPolarised,
            extra_bands: Option<ExtraBands>,
            // fix_occupancy: FixOccupancy,
        ) -> Self {
            todo!()
        }
    }
}
