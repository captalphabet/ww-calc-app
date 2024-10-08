use crate::{DiceRes, CheckParams};

/// Chain a series of dice checks with the output of each check affevcting the number of
/// rolls
// need to pass rules that affect what crits do
//
// possible effects:
// a crit in one phase may simply change the number of rolls of the next check or even a
// further upstream check
// 
// implement this generic paramter modifiy for arbitrary series of checks
//
/// Simulate should also handle full series iterarions for aggregation over many trials

pub struct Simulate {



}
