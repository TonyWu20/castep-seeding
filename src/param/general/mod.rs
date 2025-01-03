mod castep_task;
mod charge_unit;
mod data_distribution;
mod opt_strategy;
mod write_checkpoint;

use castep_task::CastepTask;
use charge_unit::ChargeUnit;
use data_distribution::DataDistribution;
use opt_strategy::OptStrategy;
use write_checkpoint::WriteCheckpoint;

#[derive(Debug, Clone, Default)]
pub struct General {
    backup_interval: i64,                // 0
    calculate_stress: bool,              // false
    calculate_densdiff: bool,            // false
    calculate_elf: bool,                 // false
    calculate_hirshfeld: bool,           // false
    charge_unit: ChargeUnit,             // E
    checkpoint: String,                  // seedname.check
    comment: String,                     // Blank
    continuation: Option<bool>,          // Null
    data_distribution: DataDistribution, // Default
    num_backup_iter: i64,                // 5
    opt_strategy: OptStrategy,           // Default
    page_wvfns: i32,                     // 0
    print_clock: bool,                   // true
    print_memory_usage: bool,            // true
    rand_seed: i32,                      // 0
    reuse: Option<bool>,                 // NULL
    run_time: i32,                       // 0
    stop: bool,                          // Example: STOP
    task: CastepTask,
    write_checkpoint: WriteCheckpoint,
    write_formatted_density: bool,   // false
    write_formatted_elf: bool,       // false
    write_formatted_potential: bool, // false
    write_orbitals: bool,            // false unless `task` is `BANDSTRUCTURE`
}
