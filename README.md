# Godel Language Specification


## Example File
```
// This is a top level comment
let x = 10;
let y = 10;
let add = fn x,y -> x+y;
let real = fn x -> if x > 0 { Ok x } else { None };
let divide = fn x,y -> if y == 0 { Error } else { Ok x / y }; 
let get_10 = fn () -> 10;
let ternary = fn () -> if current_time > 100000 { get_10 } else { 100000 };
let lists = fn x,y,z -> if x > y * z { Ok {x,y,z} } else { Error };
let multi_line = fn () -> {
  let x = 5;
  let y = 6;
  x + y
};
let strings = "This is a String";
let new_list = 0 :: lists;
add(x,y); 
real(10);
divide(10,0);
let Cell = type
| Dormant
| Alive
| Dead
;

```

Not Supported Yet:
- For loops
- Hashes
- Tagged Union 
- Evaluation




## Introduction

    Name: Godel
    Paradigm: Functional, strongly-typed, with support for algebraic data types and function piping.
    Purpose: Designed for mathematical and functional programming with a clean and expressive syntax.
    Zen: Strive to be orthogonal and linear. To be the lowest overhead and highest purity code.

## Lexical Structure

    Identifiers: Sequences of letters, digits, and underscores, starting with a letter or underscore.
    Keywords: let, fn, if, else, for, in, union, Ok, None, Error, True, False
    Operators: +, -, *, /, ==, !=, >, <, >=, <=, |>, ->
    Delimiters: {, }, (, ), [, ], :, ;, ,

## Primitive Types

    Int: Signed 64-bit width.
    Byte: Unsigned 8-bit width.
    Boolean: 0(False) or 1(True)
    String: Sequence of bytes

    You can natively cast from a Int to String

## Variable Declarations

    Syntax: let <identifier> = <expression>;
    Example:

```
    let x = 10;
    let y = 20;
```

## Functions

Let X and Y be sets, for a property P pertaining to an object x ∈ X and object y ∈ Y such that
(∀x ∈ X) => (∃!y → P(x,y) is true). Then we define the _function_ fn X → Y defined by P on 
the domain of X and rage Y for which given any x, assigns a unique map fn x → y.

    Syntax: let <identifier> = fn <parameters> -> <expression>;
    Parameters: Comma-separated list of parameters.
    Example:
```
    let sum = fn x, y -> x + y;
    let real = fn x -> if x > 0 { Ok x } else { None };
    let divide = fn x, y -> if y == 0 { Error } else { Ok x / y };
```

## Function Piping

    Syntax: <expression> |> <function>
    Example:
```
    let list_of_ints = parse filepath 
    |> buffer_to_lines 
    |> extract_int_from_lines
    |> collect_to_list;
```

## Control Flow

    If Expressions: Conditional expressions.
        Syntax: if <condition> { <expression> } else { <expression> }
        Example:

```
        let real = fn x -> if x > 0 { Ok x } else { None };
```

## For Loops (Set Builder): Iterates over a range and returns a list of results.

    Syntax: for { <variable> in <range> -> <expression> }
    Example:

```
    for { x in [0..10] -> x * x };

```

## Algebraic Data Types
    
List : Composite collection of one primitive type

        Syntax: let <type> = [ 'x(0), 'x(1), 'x(2)];
        Example:
```
        let l0= [1, 2, 3, 4];
        let l1 = 0 :: l0;
        
```

Union : Sum type that can be one of several variants.

        Syntax: let <type> = union -> | <variant> -> <type> | ... ;
        Example:
```
        let Result = union ->
        | Ok -> Int
        | None
        | Error;
```

## Matching

Matching is a powerful data inspection protocol, for almost everything except functions.

        Syntax: match <expression> in | <pattern> -> <expression> | ... ;
        Example:
```
        let Expr = union ->
        | Const -> Int
        | Add -> (Expr, Expr)
        | Mul -> (Expr, Expr)
        ;

        let eval = fn expr -> match expr in
          | Const n -> n
          | Add (e1, e2) -> eval e1 + eval e2
          | Mul (e1, e2) -> eval e1 * eval e2
        ;

```

## Expressions

    Arithmetic Expressions: +, -, *, /
    Comparison Expressions: ==, !=, >, <, >=, <=
    Function Application: <function> <arguments>
        Example:

```
        sum x y;
        real 10;
        divide 10 0;
```

## Example Programs
Example 1: Basic Arithmetic and Functions

```
let x = 10;
let y = 20;
let sum = fn x, y -> x + y;
let result = sum x y;
```

Example 2: Conditional Expressions and Result Type

```
let real = fn x -> if x > 0 { Ok x } else { None };
let divide = fn x, y -> if y == 0 { Error } else { Ok x / y };

real 10;
divide 10 0;
```

Example 3: Function Piping and Set Builder

```
let list_of_ints = parse filepath 
|> buffer_to_lines 
|> extract_int_from_lines
|> collect_to_list;


```

Example 4: Recursive Function

```
let factorial = fn n -> if n == 0 { 1 } else { n * factorial (n - 1) };
let result = factorial 5;
result;  

```

Example 5: Set-builder / For loop notation
```
let evens = for { x in [0..20] : if x % 2 == 0 { x } else { None } };
let factorial = fn n -> if n == 0 { 1 } else { n * factorial (n - 1) };
let factorials = for { x in [1..5] : factorial x };

```
