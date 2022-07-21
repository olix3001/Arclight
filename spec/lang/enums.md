### Enums

Basic enums are defined as

```
enum Enum {
    A,
    B
}
```

And are just a nice way to make your code readable, this type of enums will be compiled as integers, where in this example `Enum::A` will be compiled into `0u8`.

As it can be seen before, Values from enums can be accessed using static get operator `::`.

More complex enums can hold values. For example

```
enum Enum {
    A(i32),
    B(i64, i64)
}
```

In this example `Enum::A` can hold single `i32` value, and `Enum::B` can hold two `i64` values.
To use this enum, you write for example `Enum::A(40)`.

And to get value from `var x = Enum::A(40)`, you can use `x.0`

#### Match for enums

if we want to use `match` on the enum above, we can write

```
// x: Enum
match x {
    A (a: i32) => { ... }
    B (a: i64, b: i64) => { }
}
```
