fn main() {
    const LOWER: f64 = 0.0;
    const UPPER: f64 = 300.0;
    const STEP: f64 = 20.0;

    let mut celsius = LOWER;
    let mut fahr: f64;

    println!("Celsius - Fahrenheit table");
    while celsius <= UPPER {
        fahr = celsius * (9.0 / 5.0) + 32.0;
        println!("{:3.0} {:6.1}", celsius, fahr);
        celsius = celsius + STEP;
    }
}
