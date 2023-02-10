mod equation;
mod math;

use equation::EquationComponentType as ECT;
use equation::PartEquation;

pub fn get_version() -> &'static str {
    "0.0.1"
}

pub fn do_something() {
    let lhs1: ECT = ECT::Integer(equation::Integer { value: 12i128 });
    let rhs1: ECT = ECT::Decimal(equation::Decimal { value: 12.64f64 });

    let add_node1: ECT = ECT::AddNode(equation::AddNode {
        lhs: Box::new(lhs1),
        rhs: Box::new(rhs1),
    });

    println!("{:?}", add_node1);
    println!("{}", add_node1);

    // *** checking operators ***

    let x = equation::VariableNode { variable: 'x' };
    let pe1 = equation::PartEquation {
        eq: ECT::VariableNode(x),
    };
    let y = equation::VariableNode { variable: 'y' };
    let pe2 = equation::PartEquation {
        eq: ECT::VariableNode(y),
    };

    let eq: PartEquation = pe1 + 128.24;
    println!("Sum: {}", eq);
    println!("Sum: {:?}", eq);

    let eq: PartEquation = -eq;
    println!("Minus: {}", eq);
    println!("Minus: {:?}", eq);

    let eq: PartEquation = eq.pow(&pe2);
    println!("Power: {}", eq);
    println!("Power: {:?}", eq);

    let eq: PartEquation = eq.powi32(3);
    println!("Power2: {}", eq);
    println!("Power2: {:?}", eq);

    println!("\n\n");

    // *** Checking simplifications ***
    let x: equation::Integer = equation::Integer { value: 4 };
    let eq: PartEquation = PartEquation {
        eq: ECT::Integer(x),
    };
    let eq: PartEquation = eq + 3;
    let eq: PartEquation = eq - 6;
    let eq: PartEquation = eq * 9;
    let eq: PartEquation = 3 + eq;
    let eq: PartEquation = 6 - eq;
    let eq: PartEquation = 9 * eq;

    println!("equation: {}", eq);
    println!("simplify: {}", eq.simplify());

    println!("\n\n");

    // *** checking initializing ***
    let eq_x: PartEquation = PartEquation::new('x');
    println!("5 + x * 4 = {}", 5 + eq_x * 4);

    let i: i128 = 34;
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

    println!("\nNon Simplified: {}", eq.simplify());
    println!("Simplified: {}", eq.simplify());
}
