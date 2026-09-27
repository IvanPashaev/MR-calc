mod calculator;
mod elements;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 1 {
        println!("MR-CALC - cli utilite for calculating MR(chemistry 8 class)\n");
        println!("usage: mrcalc <formula>");
        return;
    } else if args.len() > 1 {
        let mr = args.get(1);
        println!("{:?}", mr);
        calculator::calculate(mr.unwrap().to_string());
    }
}
