//! Relational database of variant chips.



use architecture::Chip;
use crate::log::TimeReport;
use std::collections::HashMap;



pub struct VariantDatabase {
    /// List of chips belonging to the variant.
    pub(super) chip: HashMap<String, Vec<(usize, String)>>,
}

impl VariantDatabase {
    /// Creates a new `manufacturerDatabase`.
    pub(super) fn new() -> Self {
        VariantDatabase {
            chip: HashMap::new(),
        }
    }
}

/// Generates the variant database.
pub(super) async fn vars(chips: Vec<Chip>, mut report: TimeReport) -> (VariantDatabase, TimeReport) {
    let start = report.start();

    let mut vars = VariantDatabase::new();

    for (i, chip) in chips.iter().enumerate() {
        // Check the presence of the entry.
        match vars.chip.get_mut(chip.variant()) {
            Some(entry) => entry.push((i, chip.name().clone())),
            _ => { vars.chip.insert(chip.variant().clone(), vec![(i, chip.name().clone())]); },
        }
    }

    report.end(start);
    (vars, report)
}
