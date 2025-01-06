use super::KeywordDisplay;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

mod backup_setting;
mod calculate_props;
mod castep_task;
mod charge_unit;
mod checkpoint;
mod continuation;
mod data_distribution;
mod opt_strategy;
mod page_wvfns;
mod print_clock;
mod print_memory_usage;
mod rand_seed;
mod run_time;
mod stop;
mod write_checkpoint;
mod write_props;

pub use backup_setting::BackUpSetting;
pub use calculate_props::CalculateProperties;
pub use castep_task::CastepTask;
pub use charge_unit::ChargeUnit;
pub use checkpoint::Checkpoint;
pub use continuation::{Continuation, ContinueReuse, Reuse};
pub use data_distribution::DataDistribution;
pub use derive_builder::Builder;
pub use opt_strategy::OptStrategy;
pub use page_wvfns::PageWvfns;
pub use print_clock::PrintClock;
pub use print_memory_usage::PrintMemoryUsage;
pub use rand_seed::RandSeed;
pub use run_time::RunTime;
pub use stop::Stop;
pub use write_checkpoint::{WriteCheckpoint, WriteCheckpointOption, WriteCheckpointValue};
pub use write_props::WriteProperties;

#[derive(Debug, Clone, Default, Hash, Serialize, Deserialize, Builder)]
#[builder(setter(into, strip_option), default)]
pub struct General {
    backup: Option<BackUpSetting>,
    calculate_props: Option<CalculateProperties>,
    charge_unit: Option<ChargeUnit>,              // E
    checkpoint: Option<Checkpoint>,               // seedname.check
    comment: Option<String>,                      // Blank
    continuation_reuse: Option<ContinueReuse>,    // Null
    data_distribution: Option<DataDistribution>,  // Default
    opt_strategy: Option<OptStrategy>,            // Default
    page_wvfns: Option<PageWvfns>,                // 0
    print_clock: Option<PrintClock>,              // true
    print_memory_usage: Option<PrintMemoryUsage>, // true
    rand_seed: Option<RandSeed>,                  // 0
    run_time: Option<RunTime>,                    // 0
    stop: Option<Stop>,                           // Example: STOP
    task: Option<CastepTask>,
    write_checkpoint: Option<WriteCheckpoint>,
    write_props: Option<WriteProperties>,
}

impl Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = [
            self.backup.map(|v| v.output()),
            self.calculate_props.map(|v| v.to_string()),
            self.charge_unit.map(|v| v.output()),
            self.checkpoint.as_ref().map(|v| v.output()),
            self.comment.clone(),
            self.continuation_reuse.as_ref().map(|v| v.output()),
            self.data_distribution.map(|v| v.output()),
            self.opt_strategy.map(|v| v.output()),
            self.page_wvfns.map(|v| v.output()),
            self.print_clock.map(|v| v.output()),
            self.print_memory_usage.map(|v| v.output()),
            self.rand_seed.map(|v| v.output()),
            self.run_time.map(|v| v.output()),
            self.stop.map(|v| v.output()),
            self.task.map(|v| v.output()),
            self.write_checkpoint.map(|v| v.output()),
            self.write_props.map(|v| v.output()),
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

    use crate::param::{
        general::{
            calculate_props::CalculatePropertiesBuilder, write_props::WritePropertiesBuilder,
            CastepTask, GeneralBuilder, OptStrategy,
        },
        Continuation,
    };

    use super::General;

    #[test]
    fn general_keywords() {
        let g = General::default();
        let gb = GeneralBuilder::default()
            .opt_strategy(OptStrategy::default())
            .task(CastepTask::GeometryOptimization)
            .continuation_reuse(Continuation::Default)
            .write_props(
                WritePropertiesBuilder::default()
                    .formatted_elf(true)
                    .formatted_density(true)
                    .build()
                    .unwrap(),
            )
            .calculate_props(
                CalculatePropertiesBuilder::default()
                    .stress(false)
                    .densdiff(false)
                    .elf(false)
                    .hirshfeld(false)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();
        assert!(g.to_string().is_empty());
        println!("{gb}");
    }
}
