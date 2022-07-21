### Exports

Every file that has something to export, should use `export` keyword. For example:

```
// lib.arc
export module Lib {
    ...
}
```

Syntax above will export the `Lib` module from `lib.arc` file.

One file can have multiple exports, For example:

```
// lib.arc
export module A {
    ...
}

export module B {
    ...
}
```

### Imports from files

You can import from other files using `import` keyword. For example if you have file structure like

```
src
├── main.arc
└── lib.arc
```

and `lib.arc` file is:

```
// lib.arc
export module Lib {
    ...
}
```

then in `main.arc` you can import `Lib` module using

```
// main.arc
import lib::Lib;
```

or if it is

```
// lib.arc
export module Lib {
    fun lib_function(arg1: type, arg2: type): returnType { ... }
}
```

And only thing you need is `lib_function` you can use

```
// main.arc
import lib::Lib { lib_function };
```

Syntax above will only import `lib_function` from the `Lib` module.

### Imports from libraries

TODO: make some specs for this
