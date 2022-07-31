### Exports

Every file that has something to export, should use `export` keyword. For example:

```
# lib
export module Lib {
    ...
}
```

Syntax above will export the `Lib` module from `lib` file.

One file can have multiple exports, For example:

```
# lib
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
src (dir)
├── main (file)
└── lib (file)
```

and `lib` file is:

```
# lib
export module Lib {
    ...
}
```

then in `main` you can import `Lib` module using

```
# main
import lib::Lib;
```

And then values can be accessed using `Lib::value`. Or if it is

```
# lib
export module Lib {
    fun lib_function(arg1: type, arg2: type): returnType { ... }
}
```

And only thing you need is `lib_function` you can use

```
# main
import lib::Lib { lib_function };
```

Syntax above will only import `lib_function` from the `Lib` module.

### Imports from libraries

TODO: make some specs for this
