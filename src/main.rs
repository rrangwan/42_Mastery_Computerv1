use regex::Regex;
use std::env;

mod parse;
mod solve;



//parameter checks

// Check if all non-digit letters in the entire input are the same
fn check_consistent_variable(input: &str) -> bool {
    let mut variable: Option<char> = None;
    for c in input.chars() {
        if c.is_alphabetic() {
            if let Some(var) = variable {
                if c != var {
                    return false;
                }
            } else {
                variable = Some(c);
            }
        }
    }
    true
}

// Check for exactly one '='
fn check_single_equal(input: &str) -> bool {
    input.matches('=').count() == 1
}

//check if solvable term
fn is_solvable(term: &str) -> bool {
    let re = Regex::new(r"^([+-]?\d*\.?\d+)\*X\^([-+]?\d+)$").unwrap();

    re.is_match(term)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: .\\target\\debug\\computor1  \"equation\"");
        return;
    }

    //remove spaces in equation
    let equation = &args[1].replace(" ", "");

    if !check_single_equal(equation) {
        eprintln!("The input must contain exactly one '=' character.");
        return;
    }

    if !check_consistent_variable(equation) {
        eprintln!("The variable must be consistent across all terms.");
        return;
    }


    let parts: Vec<&str> = equation.split('=').collect();
    let left_terms = parse::split_terms(parts[0]);
    let right_terms = parse::split_terms(parts[1]);

    for term in left_terms {
        if !is_solvable(term){
            eprintln!("The input terms must be in form A*X^p");
            return;
        }}
        
    for term in right_terms {
        if !is_solvable(term){
            eprintln!("The input terms must be in form A*X^p");
            return;
        }}

    let coefficients = parse::parse_equation(equation);
    let reduced_form = solve::reduce_equation(&coefficients);
    println!("Reduced form: {}", reduced_form);

    let degree = coefficients.keys().cloned().max().unwrap_or(0);
    println!("Polynomial degree: {}", degree);
    if degree > 2 {
        println!("The polynomial degree is strictly greater than 2, I can't solve.");
    } else {
        let solution = solve::solve(&coefficients);
        println!("{}", solution);
    }

}
