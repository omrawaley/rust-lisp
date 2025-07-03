# Rust LISP
A simple LISP interpreter written in Rust.

## Keywords
### Operators
`+`, `-`, `*`, `/`

E.g. `(+ 1 2 3 4 5)` => `15`

### Definitions
`let` => Define a constant

E.g. `(let pi 3.14)`

`func` => Define a function

E.g. `( (func square (x) (* x x)) (square (4) )` => `16`

## To-Do
- [ ] Error handling
- [ ] Ability to interpret source files
- [ ] More keywords (e.g. `if`, `==`)
- [ ] Expanded standard library (e.g. `pow`)
