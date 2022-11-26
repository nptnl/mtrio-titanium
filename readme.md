<img src="./ti-banner.png">

---

# **Titanium:** most based math interface

So we've all seen, like, TI-89 calculators, right? I reckon that's a lofty goal.

Titanium is a my personal Rust library with the goal of providing a bridge between mathemetical capability
(see my math library, [Ferrum](https://github.com/nptnl/ferrum))
and user input.
Titanium tokenizes, parses, and evaluates simple mathemetical expressions, and is intended to be extended to the full capabilities of Ferrum, along with support for variables, equation solving, and graphing.

Many initial capabilities will be to implement Ferrum functions into the syntax, but new systems will also be created.

Currently, Titanium only works with real numbers and simple operations.

## **Syntax:**

in lisp-style, the operator is placed first before the values, and all are separated by spaces:

`* 3 3` returns `9.0`, and `+ 5 5 5` returns `15.0`.

in subtraction and division, the first value is subtracted/divided by each subsequent value:

`- 10 1 2 1` returns `6.0`, and `/ 10 5 2` returns `1.0`.

in this format, the operations become much easier to extend to other math functions.