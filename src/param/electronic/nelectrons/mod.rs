use std::fmt::Display;

use castep_seeding_derive::KeywordDisplay;
use serde::{Deserialize, Serialize};

use crate::param::KeywordDisplay;

pub use ndown::NDown;
pub use nup::NUp;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplay)]
#[keyword_display(from=f64, value=f64, field="NELECTRONS")]
pub struct NElectrons(f64);

mod nup {
    use std::fmt::Display;

    use castep_seeding_derive::KeywordDisplay;
    use serde::{Deserialize, Serialize};

    use crate::param::KeywordDisplay;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplay)]
    #[keyword_display(field="NUP", from=f64, value=f64)]
    pub struct NUp(f64);
}

mod ndown {
    use std::fmt::Display;

    use castep_seeding_derive::KeywordDisplay;
    use serde::{Deserialize, Serialize};

    use crate::param::KeywordDisplay;

    #[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd, KeywordDisplay)]
    #[keyword_display(field="NDOWN", from=f64, value=f64)]
    pub struct NDown(f64);
}
