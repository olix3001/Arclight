## Data types

### Primitives

Primitive types in Arclight are:

-   i8 (8 bit integer)
-   i16 (16 bit integer)
-   i32 (32 bit integer)
-   i64 (64 bit integer)
-   f8 (8 bit floating point number)
-   f16 (16 bit floating point number)
-   f32 (32 bit floating point number)
-   f64 (64 bit floating point number)
-   u8 (8 bit unsigned integer)
-   u16 (16 bit unsigned integer)
-   u32 (32 bit unsigned integer)
-   u64 (64 bit unsigned integer)
-   uf8 (8 bit unsigned floating point number)
-   uf16 (16 bit unsigned floating point number)
-   uf32 (32 bit unsigned floating point number)
-   uf64 (64 bit unsigned floating point number)
-   char (Single character)
-   bool (true/false)
-   str (String of characters in memory)
-   void (mainly used for return type from functions that don't return any value)
-   never (mainly used for return type from functions that never return)

#### Notes

-   Number literals can be defined as for example `5i64`, which means `5` as `i64`

### Ranges

Arclight supports ranges (type **`range`**), which are denoted as `start..end` without the `end` value, and `start..=end` with `end` value included in the range.

### Arrays

Arrays in Arclight can be defined like for example `i32[]`, which means array of
32 bit integers. It is also possible to specify array length with `i32[x]` where `x` is the length of the array.

Getting value from arrays is simply denoted as `a[index]` for `a: i32[]`.

Arrays support slicing, which for array `a: i32[]` is denoted as
`a[range]`.

Array literals as denoted as for example `[1, 2, 3]`, where all elements have the same type

### Tuples

Tuples (type **`tuple`**) are great way to encode multiple data into a single
variable. They are denoted as for example`(1, 2, 3)` and can be deconstructed using `(a, b) := (1, 2)`.

Elements from tuple (index 0 for example) can be get using `t[0]` or `t.0`

Tuple elements do not need to be of the same type.

### Dictionaries

Dictionaries (type **`dict`**) can hold data in a form of `(key, value)` pairs of unspecified types. They are denoted as:

```
{
    a: 'Hello',
    b: 'World',
}
```

and can be accessed with `dict[key]`.

### Functions/Lambdas as type

If you want to pass function/lambda as an argument you should provide `(arg1: type, arg2: type) => returnType` as an argument

### Operations

| Kind       | Defined over                     | Supported operations |
| ---------- | -------------------------------- | -------------------- |
| boolean    | **bool**                         | !, &&, \|\|, &, \|   |
| numeric    | i*, u*, f*, uf*, char            | +, -, \*, /          |
| string     | str                              | +, [i]               |
| array      | \*[]                             | +, [i], [range], ?   |
| tuple      | ( ... )                          | [i]                  |
| dict       | dict { ... }                     | [key], ?             |
| comparison | i*, u*, f*, uf*, char            | <, <=, >, >=         |
| comparison | bool, i*, u*, f*, uf*, char, str | ==, !=               |

### Type conversions

It is possible to convert between some of the types, using `value as type` notation.

### String interpolation

You can use `$` operator to identify a string literal as an interpolated string (simpler and cleaner than format).
```
var option = 1;
var name = "Name";
var x = $"Hey {name}, you chose {option} option!";
```

