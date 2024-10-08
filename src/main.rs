use ww_calc_app::*;
mod sim;
fn main() {
    let params = CheckParams{
        crit_thres: 5,
        ..Default::default()
    };

    let res = DiceRes::dice_check(10, params);

    println!("{res:?}");
}
