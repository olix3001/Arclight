## Declarations & definitions

### Notes

Function/loop/etc... body in arclight is just an expression that returns a value or not, it does not need to be a code block, it can be just an expression.

### Variable definition

variable definition is denoted as `var name: type;` (without initialization)
or `var name: type = exp;` (with initialization, type can be omitted).

### Type definition

Custom types can be defined as `typedef type_name type`. For example `typedef i32a i32[]` defines `i32a` as array of `i32`\'s.

### Function definition

Functions are defined as:

```
fun function_name(arg1: type, arg2: type): return_type body
```

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

### Returns

To return value from the code block, you just write the value as the last statement without semicolon at the end. To return from the function you should use the `ret` keyword.

### Code blocks

Code blocks are just blocks of code that can define scopes, they are denoted as `{ ... }` where `...` can be any list of expressions.
