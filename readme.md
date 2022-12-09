<img src="./ti-banner.png">

---

# **Titanium:** most based math interface

So we've all seen, like, TI-89 calculators, right? I reckon that's a lofty goal.

Titanium is a my personal Rust library with the goal of providing a bridge between mathemetical capability
(see my math library, [Ferrum](https://github.com/nptnl/ferrum))
and user input.
Titanium tokenizes, parses, and evaluates simple mathemetical expressions, and is intended to be extended to the full capabilities of Ferrum, along with support for variables, equation solving, and graphing.

Many initial capabilities will be to implement Ferrum functions into the syntax, but new systems will also be created.

Currently, Titanium's capabilities include variables and single-input mathematical functions, and will be expanded to solve equations.

## **Syntax:**

in lisp-style, the operator is placed first before the values, and all are separated by spaces:

`* 3 3` returns 9, and `+ 5 5 5` returns 15.

in subtraction and division, the first value is subtracted/divided by each subsequent value:

`- 10 1 2 1` returns 6, and `/ 10 5 2` returns 1.

for multiple operations, simply use parenthesis:

`* (+ 2 2) (- 5 3) (/ 6 3)` returns 16.

in this format, the operations become much easier to extend to other math functions.

`exp, pow, ln, log` act as operators for corresponding operators,
`ass {varname} {value}` assigns a variable to any value, and
`def {fname} {function}` creates a function using one input `'o'`.

*note: actual return values will always be of type `ferrum::ch::Comp` to handle complex-valued operations.*

---

See my small math library that is the basis for this project at
[Ferrum](https://github.com/nptnl/ferrum),
and see a few visual applications for the math at
[Chromium](https://github.com/nptnl/chromium).