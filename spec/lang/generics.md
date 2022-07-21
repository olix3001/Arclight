### Generic types

sometimes you need a function to take any type of an argument, to do this, you can take advantage of the generic type. To define it, you just put `<name>` before the arguments.

By convention, the name of the type should be `T` if there is only one type, and if not, every type should be just one capital letter.

#### Examples

```
fun add_with_cast<A, B>(a: A, b: B): A {
    return a + (b as A);
}
```

then to use this function, you just call for example

```
add_with_cast(1i64, 5i32);
```

or if you need to specify a type, because it is not in function arguments, you use

```
some_generic_function<i32>();
```
