### Classes

you can define class using

```
class MyClass { ... }
```

where `...` can contian methods, values and enums.

If you want method/value/enum to be public and available from outside of the class, you can add `public` keyword before its definition.

There is also `static` keyword, which means that this value/method is static and does not belong to any instance.

Every method in class is defined just like a function, but if it is not static, first argument that is takes, is a reference to `this`, which is just an instance of a class. For example:

```
class MyClass {
    public fun helloWorld(this: &this, arg1: type, arg2: type): returnType {
        ...
    }
}
```

classes also support constructors/destructors and operator overloads

```
// constructor
class MyClass {
    public static fun new(this: &this): MyClass {
        /*
            This is called after creating empty
            instance of the class without any of the variables set.
        */
    }
}
```

```
// destructor
class MyClass {
    public static fun drop(this: &this): void {
        /*
            This is called before removing object from the memory
        */
    }
}
```

```
// addition overload for MyClass + i32
class MyClass {
    var value: i32 = 0;

    public static fun add(this: &this, value: i32): i32
        this.value += value;
}
```

same is true for other operators:
| Operator | Function name | Argument count |
| --- | --- | --- |
| + | add | 1 |
| - | sub | 1 |
| \* | mul | 1 |
| / | div | 1 |
| ^ | pow | 1 |
| ! | not | 0 |
| & | bin_and | 1 |
| \| | bin_or | 1 |
| && | and | 1 |
| \|\| | or | 1 |
| == | eq | 1 |
| <= | le | 1 |
| >= | ge | 1 |
| < | lt | 1 |
| > | gt | 1 |
| ? | contains | 1 |
