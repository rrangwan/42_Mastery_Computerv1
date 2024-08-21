# 42_Mastery_Computerv1
42 Mastery-level Project implemented using Rust. <br>
This program takes as parameter a string with a polynomial equation problem to solve, and solves it. Solutions include imaginary number cases.

Usage:

$>make

and then

$>./target/release/computor1 "equation in correct format"

example input:
$>./target/release/computor1 "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"


example output:
Reduced form: 4 * X^0 + 4 * X^1 - 9.3 * X^2 = 0 <br>
Polynomial degree: 2 <br>
Discriminant is strictly positive, the two solutions are: <br>
0.905239 <br>
-0.475131
