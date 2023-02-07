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

    let eq: PartEquation = eq.pow(pe2);
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
}
