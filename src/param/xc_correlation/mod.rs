use std::fmt::Display;

use derive_builder::Builder;
use nlxc_options::*;
use serde::{Deserialize, Serialize};
use spin_polarised::SpinPolarised;
use xc_definition::*;
use xc_functional::XCFunctional;

use super::KeywordDisplay;

mod nlxc_options;
mod spin_polarised;
mod xc_definition;
mod xc_functional;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct XcParam {
    xc_functional: Option<XCFunctional>,
    xc_definition: Option<XCDefinition>,
    spin_polarised: Option<SpinPolarised>,
    nlxc_options: Option<NLXCOptions>,
}

impl Display for XcParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.xc_functional.map(|v| v.output()),
            self.xc_definition.as_ref().map(|s| s.to_string()),
            self.spin_polarised.map(|v| v.output()),
            self.nlxc_options.map(|s| s.to_string()),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<String>>()
        .join("\n");
        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod test {
    use super::{
        nlxc_options::NLXCOptionsBuilder, xc_functional::XCFunctional, NLXC_PPD_Int,
        XCDefinitionBuilder, XcParamBuilder,
    };

    #[test]
    fn xc_param() {
        let xc_param = XcParamBuilder::default()
            .xc_functional(XCFunctional::HSE03)
            .spin_polarised(true)
            .nlxc_options(
                NLXCOptionsBuilder::default()
                    .impose_trs(false)
                    .build()
                    .unwrap(),
            )
            .xc_definition(
                XCDefinitionBuilder::default()
                    .nlxc_ppd_int(NLXC_PPD_Int::On)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        println!("{}", xc_param);
    }
}
