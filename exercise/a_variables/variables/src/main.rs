fn main() {
    const STARTING_MISSILES: i32 = 8;
    const READY_AMOUNT: i32 = 2;

    let (mut missiles, ready): (i32, i32) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);

    missiles = missiles - ready;
    println!("{} missiles left", missiles);

    // This warns that missiles does not need to be mutable.
    // println!("{} missiles left", missiles - ready);

    // This warns that extra is an unused variable.
    // let extra = 5;

    // This warns invalid left-hand side of assignment
    // READY_AMOUNT = 3;
}
