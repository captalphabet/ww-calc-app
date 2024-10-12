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
/// SimulationChain should also handle full series iterarions for aggregation over many trials
///

pub struct SimulationChain {
    tests: Vec<SimulationLink>,



}

impl SimulationChain {
    pub fn new() -> Self {
        Self{
            tests: Vec::new(),
        }



    }

    pub fn _insert_pre_attack(&mut self, num: usize, pass_thres: usize ) {
        self.tests.push(
            SimulationLink {
                check_num: num,
                crit_effect: None,
                param: CheckParams {
                    pass_thres,
                    ..Default::default()
                }
            }
        );
        
    }
    // Takes all data for the attacker and defender and outputs the total
    pub fn attack_sequence(&mut self, attacker: OffProfile, defender: DefProfile ) -> Vec<usize> {

    }

}

#[derive(Default)]
pub struct SimulationLink {
    check_num: usize,
    crit_effect: Option<CritEffect>,
    param: CheckParams,
}


pub enum CritEffect {
    Bonus(usize), // extra passes to next text in chain
    Bypass(usize), // crits bypass next check in chain to the usize th following, eg Bypass(1)
                   // skips one test and adds the crits to that test

}
