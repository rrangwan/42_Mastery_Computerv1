use regex::Regex;
use std::collections::HashMap;

//imports the Regex type from the regex crate.
//regex is used for matching patterns with expected formatting, like re variable below

//function takes a string slice and returns a vector of strings
// function splits the equation according to +/- signs, except for the first character
pub fn split_terms(expression: &str) -> Vec<&str> {
    let mut terms = Vec::new();
    //mut modifier allows the variable to change, else by default cannot change
    let mut start = 0;
    for (i, c) in expression.char_indices() {
        if (c == '+' || c == '-') && i != 0 {
            terms.push(&expression[start..i]);  //passing expression as reference, memmory efficient, does not create new string
            start = i;
        }
    }
    terms.push(&expression[start..]);
    terms //returns terms
}


//This function is designed to parse individual terms of a polynomial expression, extracting the coefficient and power of each term.
pub fn parse_term(term: &str) -> (f64, i32)
//take string slice and returns tuple for floating 64 and int 32
{
    let re = Regex::new(r"^([+-]?\d*\.?\d+)\*X\^([-+]?\d+)$").unwrap();
    //unwrap() for catching errors if result not true, if pattern possible will return 'Ok(instance)' else will return 'Err(error)'
    //since confident there will be no error, no error handling instructions, only for catching on compilation purposes.
    //r for treating things with \ as litereal
    //([+-]?  is either + or -, and is optional
    //\d* 0-9
    //\.? literal decimal point, optional
    //\*X \^  literal *X^
    //(\d+) 0-9 at least one digit
    //The regex pattern matches terms like "±A * X^b"

    if let Some(caps) = re.captures(term) {
        //some, is option type (enum from standard rust library, can be some(T) or None), if there is a match betweeb term's formating and re's formatting then proceed else returns null
        let coefficient = caps[1].replace(' ', "").parse::<f64>().unwrap_or(0.0);
        //capturing group index start from 1 not 0, caps[0] is whole string
        //our cap groups are: ([+-]?\s*\d*\.?\d*)  and (\d+)
        //sets coefficient, replace removes spaces, changes type to floating 64, or makes it 0.0
        let power = caps[2].parse::<i32>().unwrap_or(0);
        return (coefficient, power);
        //sets power, changes type to int 32, or makes it 0
    }
    (0.0, 0) //return default 0  for coefficient and power if wrong formatting
}




pub fn parse_equation(equation: &str) -> HashMap<i32, f64>
//returns Hashmap a standard collection, with key value pairs.
//key is power, value is coefficient
//easy to put all coefficients together according to power of (to reduce equation)
{
    let parts: Vec<&str> = equation.split('=').collect();
    let left_terms = split_terms(parts[0]);
    let right_terms = split_terms(parts[1]);

    let mut coefficients = HashMap::new();

    for term in left_terms {
        if !term.trim().is_empty() {
            let (coeff, power) = parse_term(term);
            *coefficients.entry(power).or_insert(0.0) += coeff; //insert 0.0 if needed
            // * dereferencing so actual value is modified
        }
    }

    for term in right_terms {
        if !term.trim().is_empty() {
            let (coeff, power) = parse_term(term);
            *coefficients.entry(power).or_insert(0.0) += -1.0 * coeff;
        }
    }
    //reverse sign and add up coefficients by power

    coefficients
}


