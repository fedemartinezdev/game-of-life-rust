use std::env::{args, Args};

fn calculate(op1: f32, op: char, op2:f32) -> f32{
    match op{
        '+' => op1 + op2,
        '-' => op1 - op2,
        '*' => op1 * op2,
        '/' => op1 / op2,
        _ => panic!("Invalid operator")
    }
}

fn main(){
    let mut args: Args = args();
    let operand1 = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let operand2 = args.nth(0).unwrap();

    let operand1f = operand1.parse::<f32>().unwrap();
    let operand2f = operand2.parse::<f32>().unwrap();

    let result = calculate(operand1f, operator, operand2f);

    print!("{}{}{}={}", operand1f, operator, operand2f, result);
}