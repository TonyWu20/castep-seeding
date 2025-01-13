# castep-seeding

Generate `castep` seed files

## Development track

- Dec 23, 2024: First version of design is done. Based around traits `RootJob`, `SeedFolder`, `CellBuilding` and `ParamBuilding`.
  Data struct of `CastepParams` is imported from `castep-cell-io`, but the code
  design of it needs to be rewritten from `Vec<Enum>` type to a full field
  struct with defaults.

- Parameter keyword sections
  - [x] General parameters
  - [x] Units
  - [x] Exchange-correlation parameters
  - [x] Pseudopotentials
  - [x] Basis set parameters
  - [x] Electronic parameters (partially completed)
  - [x] Electronic minimization parameters
  - [x] Density mixing parameters
  - [x] Population analysis
  - [x] Geometry optimization parameters
  - [x] Band structure parameters
  - [ ] Molecular dynamics
  - [ ] Optics
  - [ ] Transition state search
  - [ ] Phonon parameters
  - [ ] Electric field parameters
  - [ ] NMR parameters
  - [ ] Excitation energies parameters
  - [ ] Solvation energy parameters
