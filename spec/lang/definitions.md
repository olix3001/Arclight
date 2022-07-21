## Declarations & definitions

### Notes

Function/loop/etc... body in arclight is just an expression that returns a value or not, it does not need to be a code block, it can be just an expression.

### Variable definition

variable definition is denoted as `var name: type;` (without initialization)
or `var name: type = exp;` (with initialization, type can be omitted).

Variables can also be constant, to define such variable you are ought to use `const name: type = const_exp;` syntax. Constant expressions are things that don't change during the runtime of a program. Examples of such expressions are literals.

### Type definition

Custom types can be defined as `typedef type_name type`. For example `typedef i32a i32[]` defines `i32a` as array of `i32`\'s.

### Function definition

Functions are defined as:

```
fun function_name(arg1: type, arg2: type): return_type body
```

Two functions with the same name can exist if they take different number or different types of arguments.

### If statement definition

If statements are defined as:

```
if (expr) body
```

or

```
if (expr) body else body
```

Both of those return value from the body block.

### Loops

Basic loop is the infinite one, it can be written as

```
loop body
```

And loop with an expression, written as

```
loop (expr) body
```

But there are also for loops, that can go over a range

```
for (name in range) body
```

And for loops that are known from many other programming languages

```
for (expr; expr; expr) body
```

In loops `break` and `continue` keywords can be used.

### Returns

To return value from the code block, you just write the value as the last statement without semicolon at the end. To return from the function you should use the `ret` keyword.

### Code blocks

Code blocks are just blocks of code that can define scopes, they are denoted as `{ ... }` where `...` can be any list of expressions.

### Match expression

You can use match expression, which works similar to `switch` statement in many other languages. Is is used like

```
// Let's assume that x is an integer
match x {
    0..=2 => 0,

    5
    | 10..=15 => 1

    _ => 2
}
```

This will return 0 if `x` is between 0 and 2, return 1 if `x` is 5 or is between 10 and 15, and return 2 in every other scenario.
