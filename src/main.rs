use std::collections::HashMap;

use rand::{thread_rng, Rng};


fn main() {
    let  params = CheckParams {
        reroll_thres: Some(ReRollCond::One),
        ..Default::default()
    };
    let res = DiceRes::dice_check(10, params);

    dbg!(&res);
}

// calculate expected damage from arbitrary number of dice checks given varying pass conditions
// eg. 5 models has 2 attacks each with a given strength, ap, and damage profile, against a target
// with a given defensive profile, repeat n times and aggregate results
// optionally combine with other dice series tests
// Model: n dice ->  [check A] -> x_a passes -> [check B] -> x_b passes -> [check C] etc.. repeat
// N_sim times
// a test needs:
// - number of checks
// - threshold to pass
// - optional re-roll threshold
// - return number of passes and 'critical passes'

enum ReRollCond {
    Thres(usize),
    One,
}

struct CheckParams {
    pass_thres: usize,
    reroll_thres: Option<ReRollCond>,
    crit_thres: usize,
}

impl Default for CheckParams {
    fn default() -> Self {
        Self {
            pass_thres: 3,
            reroll_thres: None,
            crit_thres: 6,
        }
    }
}

/// Holds set of dice checks
#[derive(Debug)]
struct DiceRes {
    passes: usize,
    crits: usize,
}

impl DiceRes {
    fn dice_check(num: usize, params: CheckParams) -> Self {
        let mut roll_counts: HashMap<usize, usize> = HashMap::new();
        let mut passes: usize = 0;
        let mut crits: usize = 0;

        enumerate_rolls(&mut roll_counts, num, dice_roll);

        // checks for critical successes and saves the failed rolls for potential re rolls
        let (mut passed_checks, mut failed_checks): (HashMap<usize,usize>, HashMap<usize,usize>) = roll_counts
            .iter()
            .partition(|(roll, _count)| **roll >= params.pass_thres);

        // to handle re rolls we can compute the number of dice to roll and merge the result with
        // passed_checks
        // Need to handle all the re roll cases
        // ReRollCond::One -> roll one dice ONLY if at least one fail
        // ReRoll::Thres(val) -> roll the sum of failed dice that satisfy the re roll threshold

        


        
        
        
        
        





        passed_checks.iter().for_each(|(roll, count)| {
            if *roll >= params.crit_thres {
                crits += *count;
            } else {
                passes += *count;
            }
        });

        Self { passes, crits }
    }

    const fn total_pass(&self) -> usize {
        self.crits + self.passes
    }
}

// modifies a hashmap of rolls and aggregates the data
// Decoupled dice_roll from count
fn enumerate_rolls<T>(map: &mut HashMap<usize, usize>, num: usize, mut num_gen: T)
where
    T: FnMut() -> usize,
{
    (0..num).map(|_| num_gen()).for_each(|roll| {
        map.entry(roll).and_modify(|count| *count += 1).or_insert(1);
    });
}

// 6 - sided dice
fn dice_roll() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(1..7)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{dice_roll, enumerate_rolls};

    // Check dice_roll has correct bounds
    #[test]
    fn bounds() {
        let rolls: Vec<usize> = (0..100).map(|_| dice_roll()).collect();
        // checking dice rolls are within bounds for a 6sided die,
        assert!(rolls.iter().max().unwrap() <= &6 && rolls.iter().min().unwrap() >= &1)
    }

    //check enumerate_rolls counts properly
    #[test]
    fn count_roll() {
        let mut data: Vec<usize> = [1, 2, 3, 3, 3, 2, 5, 6, 1].into();

        let mut counts: HashMap<usize, usize> = HashMap::new();
        enumerate_rolls(&mut counts, data.len(), move || data.pop().unwrap_or(0));

        assert!(*counts.get(&1).unwrap_or(&0_usize) == 2);
        assert!(*counts.get(&2).unwrap_or(&0_usize) == 2);
        assert!(*counts.get(&3).unwrap_or(&0_usize) == 3);
        assert!(*counts.get(&4).unwrap_or(&0_usize) == 0);
        assert!(*counts.get(&5).unwrap_or(&0_usize) == 1);
        assert!(*counts.get(&6).unwrap_or(&0_usize) == 1);
    }
}
