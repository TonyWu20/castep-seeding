# castep-seeding

Generate `castep` seed files

## Development track

- Dec 23, 2024: First version of design is done. Based around traits `RootJob`, `SeedFolder`, `CellBuilding` and `ParamBuilding`.
  Data struct of `CastepParams` is imported from `castep-cell-io`, but the code
  design of it needs to be rewritten from `Vec<Enum>` type to a full field
  struct with defaults.
