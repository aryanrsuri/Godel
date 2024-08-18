# Gödel Language Specification

## Introduction

    Name: Gödel
    Purpose: Functional and mathematical PL with a pure syntax.
    Zen: Strive to be orthogonal and linear. To be the lowest overhead and highest purity code.

## Example File
```
// This is a top level comment
let x = 10;
let y = 10;
let add = fn x,y -> x+y;
let real = fn x -> if x > 0 { Ok x } else { None };
let divide = fn x,y -> if y == 0 { Error } else { Ok x / y };
let mask = fn x,y -> if x == y { 0 } else { x % y };
let get_10 = fn () -> 10;
let ternary = fn () -> if current_time > 100000 { get_10 } else { 100000 };
let result_array = fn x,y,z -> if x > y * z { Ok {x,y,z} } else { Error };
let factorial = fn n -> if n == 0 { 1 } else { n * factorial (n - 1) };
let multi_line = fn () -> {
  let x = 5;
  let y = 6;
  x + y
};
let strings = "This is a String";
let list = {0, 1, 2};

// Sets are generated with implicit for systnax { x : [0..2] -> x }
// Or by casting a list to set:
// let set_from_list = {0, 1, 1, 3}.set
// $ {0, 1, 3}

let array = [1, 2];
let new_list = 0 :: list;
let cardinality = #{0, 1, 2};
let elem = array.0;
add(x,y); 
real(10);
divide(10,0);
let Cell = type
| Dormant 
| Alive
| Dead
| Health 
;

```

## Not Supported Yet (From Below Spec)
- For / Set-builder notation
- Sets
- Hashes
- Function piping
- Pattern Matching types
- Union primitive types i.e. Tagged Unions
- Evaluation
- Array access

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
    let evens = for { x in [0..20] : if x % 2 == 0 { x } else { None } };
    let factorial = fn n -> if n == 0 { 1 } else { n * factorial (n - 1) };
    let factorials = for { x in [1..5] : factorial x };

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

        Syntax: let <type> = type | <variant> -> <type> | ... ;
        Example:
```
        let Result = type
        | Ok -> Int
        | None
        | Error
        ;
```

## Matching

Matching is a powerful data inspection protocol, for almost everything except functions.

        Syntax: match <expression> in | <pattern> -> <expression> | ... ;
        Example:
```
        
        let Cell = type
        | Alive Int
        | Dead Int
        | Dormant
        ;
        
        let partition = fn cell -> match cell 
        | Alive n -> attack(n) 
        | Dead n -> revive(n)
        | Dormant () -> ()
        ;

        let birth = fn () -> Cell.Dead : 0 ;
        let Cells = for { x <- [0..64] : birth }
        for { cell <- Cells : partition(cell) }

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
