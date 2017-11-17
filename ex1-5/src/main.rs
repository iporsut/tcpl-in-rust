fn main() {
    const LOWER: f64 = 0.0;
    const UPPER: f64 = 300.0;
    const STEP: f64 = 20.0;

    let mut fahr = UPPER;
    let mut celsius: f64;

    println!("Fahrenheit - Celsius table");
    while fahr >= LOWER {
        celsius = 5.0 * (fahr - 32.0) / 9.0;
        println!("{:3.0} {:6.1}", fahr, celsius);
        fahr = fahr - STEP;
    }
}
