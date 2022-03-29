//! Relational database of families chips.



use architecture::Chip;
use crate::log::TimeReport;
use std::collections::HashMap;



pub struct FamilyDatabase {
    /// List of chips belonging to the family.
    pub(super) chip: HashMap<String, Vec<(usize, String)>>,

    /// List of variants belonging to the manufacturer.
    pub(super) vars: HashMap<String, Vec<(usize, String)>>,
}

impl FamilyDatabase {
    /// Creates a new `manufacturerDatabase`.
    pub(super) fn new() -> Self {
        FamilyDatabase {
            chip: HashMap::new(),
            vars: HashMap::new(),
        }
    }
}

/// Generates the family database.
pub(super) async fn fmls(chips: Vec<Chip>, mut report: TimeReport) -> (FamilyDatabase, TimeReport) {
    let start = report.start();

    let mut fmls = FamilyDatabase::new();

    for (i, chip) in chips.iter().enumerate() {
        // Check the presence of the entry.
        match fmls.chip.get_mut(chip.family()) {
            Some(entry) => entry.push((i, chip.name().clone())),
            _ => { fmls.chip.insert(chip.family().clone(), vec![(i, chip.name().clone())]); },
        }

        // Check the presence of the entry.
        match fmls.vars.get_mut(chip.family()) {
            Some(entry) => entry.push((i, chip.variant().clone())),
            _ => { fmls.vars.insert(chip.family().clone(), vec![(i, chip.variant().clone())]); },
        }
    }

    report.end(start);
    (fmls, report)
}
