### Interfaces

You can also define an interface, which tells function, what should it implement

```
interface MyInterface {
    public fun some_function(arg1: i32): f64;

    var some_variable: u8;
}
```

This means, that if class implements this interface (`class MyClass implements MyInterface { ... }`) then it needs to implement public method named `some_function` with specified arguments and return type as well as `some_variable` with type `u8`
