# Arclight std::io module

This module contains everything connected to input/output.

## print/println

One of the main functionalities of this module is the `print` or `println` function.

! println is just a print that adds new line at the end

This function takes string literal and optionally arguments to format the string. For example:

```
import std::io { println };

const name = "Arclight";

fun main(): void {
    println("Hello {}!", name);
}
```

It can also use named formats. For example:

```
import std::io { println };

fun main(): void {
    println("Hello {name}!", {
        name: "Arclight",
    });
}
```

There are other types of arguments:
| placeholder | types | name |
| --- | --- | --- |
| {} | i\*, f\*, u\*, bool, str | value argument |
| {:?} | array, tuple, vector, etc... | debug value |

## format

Another functionality is the `format` function, it works just like print formatting, except that it returns string instead of printing it

```
import std::io { format, println };

const name = "Arclight";

fun main(): void {
    var formatted = format("Hello {}!", name);
    println(formatted);
}
```

## Input

Sometimes you need to get user input. To do this, you can use `input` function, which allows us to get value (string) from the user.

```
import std::io { input, print, println };

fun main(): void {
    print("What is your name? ");
    var name = input();

    println("Hello {}!", name);
}
```
