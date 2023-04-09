use math_engine;
use math_engine::equation::PartEquation;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("MathEngine::Version {}", math_engine::get_version());

    do_something();

    ExitCode::SUCCESS
}

pub fn do_something() {
    // *** Checking simplifications ***
    let x: PartEquation = PartEquation::new('x');

    // *** checking initializing ***
    println!("5 + x * 4 = {}", 5 + &x * 4);

    let i: i64 = 34;
    let eq_i: PartEquation = 5 + PartEquation::newi(i) * 4;
    let eq_i_s: PartEquation = eq_i.simplify();
    let eq_i: PartEquation = 5 + PartEquation::newi(i) * 4;
    println!(
        "5 + x * 4 = {}, where x = {} \t simplified: {}",
        eq_i, i, eq_i_s
    );

    let d: f64 = 3.1415;
    let eq_d: PartEquation = 5 + PartEquation::newf(d) * 4.2;
    let eq_d_s: PartEquation = eq_d.simplify();
    let eq_d: PartEquation = 5 + PartEquation::newf(d) * 4.2;
    println!(
        "5 + x * 4.2 = {}, where x = {} \t simplified: {}",
        eq_d, d, eq_d_s
    );

    // *** check operations based on reference ***
    let x: PartEquation = PartEquation::new('x');
    let y: PartEquation = PartEquation::new('y');
    let z: PartEquation = PartEquation::new('z');
    let two: PartEquation = PartEquation::newi(2);

    let eq: PartEquation = 14 - &x + &x * 65 / 24;

    println!("{} -> {}", x, eq);

    // *** substitution ***
    let eq2: PartEquation = eq.simplify();
    let eq3: PartEquation = eq.substitutei('x', 2);

    println!("eq: {}, eq2: {}", eq, eq2);
    println!("eq, 2 for x: {}", eq3);

    // *** updated simplification ***
    println!("\n");
    let eq: PartEquation = &x + &x + &x + 5 + 4 + 3 + 2;
    let eq_error: PartEquation = &x + &x + &x + 5 + 4 + 3 + 2 * &eq_i * 4;
    let eq_minus: PartEquation = &x - &x - &x - 5 - 4 - 3 - 2;
    let eq2: PartEquation =
        &x + &x + &x + 5 + 4 + 3 + 2 * &eq_i * 4 + &y + &z + (&z * 6) + (&y / 5);
    println!("Non Simplified: {}", eq);
    println!("Simplified: {}", eq.simplify());
    println!("Non Simplified: {}", eq_minus);
    println!("Simplified: {}", eq_minus.simplify());

    println!("\nNon Simplified: {}", eq_error);
    println!("Simplified: {}", eq_error.simplify());

    println!("\nNon Simplified: {}", eq2);
    println!("Simplified: {}", eq2.simplify());

    let eq: PartEquation = 3 * &x * &x * &x * 7 * &two;
    println!("\nNon Simplified: {}", eq);
    println!("Simplified: {}", eq.simplify());

    println!("\n\n{}", x.pow(&y.pow(&z)));
    println!("\n\n{}", x.pow(&y).pow(&z));

    // *** checking substitution of floats ***
    let eq: PartEquation = 1 + &x + &y;
    println!(
        "\nEq: {}, substituted {} with {} -> {}",
        eq,
        'x',
        2.0,
        eq.substitutef('x', 2.0)
    );

    let eq: PartEquation = 2 * x.pow(&y);
    println!(
        "\nEq: {}, substituted {} with {} -> {}",
        eq,
        'x',
        2.0,
        eq.substitutef('x', 2.0)
    );

    // *** check pre_simplification ***
    let eq: PartEquation = 1 + &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = 1 - &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq = -(-(-&x));
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = 1 * 4 / &y * &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = 1 + &x - 1;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = 0.5 * &x * 2;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = (3 * &x) + &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = (3 * &x) + (&x * 5);
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = &x + &x + &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = x.powi64(3) * x.powi64(2) * &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = x.powi64(3) * &x;
    println!("\n{} -> {}", eq, eq.simplify());

    let eq: PartEquation = x.powi64(3).powi64(7);
    println!("\n{} -> {}", eq, eq.simplify());
}
