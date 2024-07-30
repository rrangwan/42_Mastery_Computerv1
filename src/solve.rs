
use std::collections::HashMap;


//function takes a reference to a HashMap of coefficients and returns a String.
// returns the reduced form as a string to be printed to console
pub fn reduce_equation(coefficients: &HashMap<i32, f64>) -> String {

    //returns interations to references in the hashmap, (hashmap still retains ownership, Hashmap is not being mutated)
    //which is then collected as vector tupple types.
    //()at the end is just calling collect method
    let mut terms = coefficients.iter().collect::<Vec<(&i32, &f64)>>();

     //sorts the terms by power in ascending order.
    //sort_by_key is built in Rush standard
    //the 'key' is the 'power' at the end, it is the first element of the tupple '|..|', second element '_' means ignore second element
    //the | | is a closure argument list, in this case its a tupple (a,b)
    terms.sort_by_key(|&(power, _)| power);

    let mut reduced_terms = Vec::new();
    for (power, coeff) in terms {
        if *coeff != 0.0 {
            reduced_terms.push(format!("{} * X^{}", coeff, power));
        }
    }

    if reduced_terms.is_empty() {
        return "0 = 0".to_string();
    }

    //Joins all terms with " + " and then
    // replaces "+ -" with "- " to handle negative terms
    reduced_terms.join(" + ").replace("+ -", "- ") + " = 0"
}


// takes two parameters value, which is the number whose square root is to be found, and epsilon, which is the tolerance level for the approximation.
//it returns either Some(f64) if the calculation is successful or None if it is not.
//we will guess the sqrt in a loop
pub fn sqrt(value: f64, epsilon: f64) -> Option<f64> {
    if value < 0.0 {
        return None;
    }
    let mut guess = value / 2.0;
    while (guess * guess - value).abs() > epsilon {
        guess = (guess + value / guess) / 2.0;
        //this  formula comes from the Babylonian method.
    }
    Some(guess)
}



pub fn solve_quadratic(a: f64, b: f64, c: f64) -> String {
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        // Discriminant is negative, so we have complex solutions
        let real_part = -b / (2.0 * a);
        let imaginary_part = sqrt(-discriminant, 1e-10).unwrap() / (2.0 * a);
        return format!(
            "Discriminant is strictly negative, the two complex solutions are:\n{} + {}i\n{} - {}i",
            real_part, imaginary_part, real_part, imaginary_part
        );
     } else if discriminant == 0.0 {
        let x = -b / (2.0 * a);
        //format! is a Rust macro the returns a formatted string, but does not print it to console
        // \n new line
        // {} placeholder for variable x
        return format!("Discriminant is zero, the solution is:\n{}", x);
    } else {
        //unwrap returns error if sqrt does not return f64
        let sqrt_discriminant = sqrt(discriminant, 1e-10).unwrap();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);
        return format!("Discriminant is strictly positive, the two solutions are:\n{}\n{}", x1, x2);
    }
}



pub fn solve(coefficients: &HashMap<i32, f64>) -> String {

    //extracts the coefficients a, b, and c from a HashMap<i32, f64>. If a coefficient is not present, it defaults to 0.0.
    //example ax^2 +bx + c (it is stored in ascending order in the hashmap)
    let a = *coefficients.get(&2).unwrap_or(&0.0);
    let b = *coefficients.get(&1).unwrap_or(&0.0);
    let c = *coefficients.get(&0).unwrap_or(&0.0);

    if a != 0.0 {
        // is not zero, it is a quadratic equation, and the solve_quadratic function is called.
        solve_quadratic(a, b, c)
    } else if b != 0.0 {
        //If a is zero and b is not zero, it is a linear equation of the form
        //bx + c = 0, and x = -c/b
        let x = -c / b;
        format!("The solution is:\n{}", x)
    } else if c != 0.0 {
        //if both a and b are zero and c is not zero, there is no solution because the equation reduces to c = 0 which is false.
        "No solution.".to_string()
    } else {
        // any real number is a solution because 0 =0 is true for all R
        "All real numbers are solutions.".to_string()
    }
}
