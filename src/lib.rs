use std::collections::HashMap;
use rand::{thread_rng, Rng};

pub struct CheckParams {
    pub pass_thres: usize,
    pub reroll_thres: Option<ReRollCond>,
    pub crit_thres: usize,
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

#[allow(dead_code)]
pub enum ReRollCond {
    Thres(usize),
    One,
}

#[derive(Debug)]
pub struct DiceRes {
    pub passes: usize,
    pub crits: usize,
}

impl DiceRes {
    pub fn dice_check(num: usize, params: CheckParams) -> Self {
        let mut roll_counts: HashMap<usize, usize> = HashMap::new();
        let mut passes: usize = 0;
        let mut crits: usize = 0;

        enumerate_rolls(&mut roll_counts, num, dice_roll);

        let (passed_checks, failed_checks): (HashMap<usize, usize>, HashMap<usize, usize>) =
            roll_counts
                .iter()
                .partition(|(roll, _count)| **roll >= params.pass_thres);

        let mut reroll_count = 0;

        if let Some(reroll_cond) = &params.reroll_thres {
            match reroll_cond {
                ReRollCond::One => {
                    if !failed_checks.is_empty() {
                        reroll_count = 1;
                    }
                }
                ReRollCond::Thres(val) => {
                    for (roll, count) in &failed_checks {
                        if *roll <= *val {
                            reroll_count += count;
                        }
                    }
                }
            }
        }

        let mut rerolled_counts: HashMap<usize, usize> = HashMap::new();
        enumerate_rolls(&mut rerolled_counts, reroll_count, dice_roll);

        rerolled_counts
            .iter()
            .filter(|(roll, _count)| **roll >= params.pass_thres)
            .for_each(|(roll, count)| {
                if *roll >= params.crit_thres {
                    crits += *count;
                } else {
                    passes += *count;
                }
            });

        passed_checks.iter().for_each(|(roll, count)| {
            if *roll >= params.crit_thres {
                crits += *count;
            } else {
                passes += *count;
            }
        });

        Self { passes, crits }
    }

    #[allow(dead_code)]
    pub const fn total_pass(&self) -> usize {
        self.crits + self.passes
    }
}

fn enumerate_rolls<T>(map: &mut HashMap<usize, usize>, num: usize, mut num_gen: T)
where
    T: FnMut() -> usize,
{
    (0..num).map(|_| num_gen()).for_each(|roll| {
        map.entry(roll).and_modify(|count| *count += 1).or_insert(1);
    });
}

pub fn dice_roll() -> usize {
    let mut rng = thread_rng();
    rng.gen_range(1..=6)
}

#[cfg(test)]
mod tests {
    use super::{dice_roll, enumerate_rolls};
    use std::collections::HashMap;

    #[test]
    fn bounds() {
        let rolls: Vec<usize> = (0..100).map(|_| dice_roll()).collect();
        assert!(rolls.iter().max().unwrap() <= &6 && rolls.iter().min().unwrap() >= &1)
    }

    #[test]
    fn count_roll() {
        let mut data: Vec<usize> = [1, 2, 3, 3, 3, 2, 5, 6, 1].into();

        let mut counts: HashMap<usize, usize> = HashMap::new();
        enumerate_rolls(&mut counts, data.len(), move || data.pop().unwrap_or(0));

        assert_eq!(*counts.get(&1).unwrap_or(&0_usize), 2);
        assert_eq!(*counts.get(&2).unwrap_or(&0_usize), 2);
        assert_eq!(*counts.get(&3).unwrap_or(&0_usize), 3);
        assert_eq!(*counts.get(&4).unwrap_or(&0_usize), 0);
        assert_eq!(*counts.get(&5).unwrap_or(&0_usize), 1);
        assert_eq!(*counts.get(&6).unwrap_or(&0_usize), 1);
    }
}
