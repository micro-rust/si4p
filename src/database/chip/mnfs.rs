//! Relational database of manufacturer chips.



use architecture::Chip;
use crate::log::TimeReport;
use std::collections::HashMap;



pub struct ManufacturerDatabase {
    /// List of chips belonging to the manufacturer.
    pub(super) chip: HashMap<String, Vec<(usize, String)>>,

    /// List of families belonging to the manufacturer.
    pub(super) fmls: HashMap<String, Vec<(usize, String)>>,

    /// List of variants belonging to the manufacturer.
    pub(super) vars: HashMap<String, Vec<(usize, String)>>,
}

impl ManufacturerDatabase {
    /// Creates a new `manufacturerDatabase`.
    pub(super) fn new() -> Self {
        ManufacturerDatabase {
            chip: HashMap::new(),
            fmls: HashMap::new(),
            vars: HashMap::new(),
        }
    }
}

/// Generates the manufacturer database.
pub(super) async fn mnfs(chips: Vec<Chip>, mut report: TimeReport) -> (ManufacturerDatabase, TimeReport) {
    let start = report.start();

    let mut mnfs = ManufacturerDatabase::new();

    for (i, chip) in chips.iter().enumerate() {
        // Check the presence of the entry.
        match mnfs.chip.get_mut(chip.manufacturer()) {
            Some(entry) => entry.push((i, chip.name().clone())),
            _ => { mnfs.chip.insert(chip.manufacturer().clone(), vec![(i, chip.name().clone())]); },
        }

        // Check the presence of the entry.
        match mnfs.fmls.get_mut(chip.manufacturer()) {
            Some(entry) => entry.push((i, chip.family().clone())),
            _ => { mnfs.fmls.insert(chip.manufacturer().clone(), vec![(i, chip.family().clone())]); },
        }

        // Check the presence of the entry.
        match mnfs.vars.get_mut(chip.manufacturer()) {
            Some(entry) => entry.push((i, chip.variant().clone())),
            _ => { mnfs.vars.insert(chip.manufacturer().clone(), vec![(i, chip.variant().clone())]); },
        }
    }

    report.end(start);
    (mnfs, report)
}
