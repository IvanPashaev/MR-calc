use crate::elements::ELEMENTS;

fn get_mass(symbol: &str) -> Option<f64> {
    ELEMENTS
        .iter()
        .find(|(s, _)| *s == symbol)
        .map(|(_, mass)| *mass)
}

pub fn calculate(formula: String) {
    let mut symbols = formula.chars().peekable();
    let mut answer = 0.0;
    while let Some(c) = symbols.next() {
        let mut doubls = 1;
        if c.is_uppercase() {
            let mut symbol: String = String::new();
            if let Some(&next) = symbols.peek() {
                if next.is_lowercase() {
                    symbol.insert(0, c);
                    symbol.insert(1, next);
                    symbols.next();
                    if let Some(&next) = symbols.peek() {
                        if next.is_ascii_digit() {
                            doubls = next.to_digit(10).unwrap();
                            symbols.next();
                        }
                    }
                } else {
                    symbol.insert(0, c);
                    if next.is_ascii_digit() {
                        doubls = next.to_digit(10).unwrap();
                        symbols.next();
                    }
                }
            } else {
                symbol.insert(0, c);
            }
            answer += get_mass(&symbol).unwrap().round() * (doubls as f64);
        }
    }
    println!("answer: {}", answer);
}
