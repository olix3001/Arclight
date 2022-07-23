import std::io { input, print, println };
import std::conversion { parse_i32 };

fun print_fibonacci(n: i32): {
    var t1 = 0;
    var t2 = 1;
    var next_term = t1 + t2;

    for (i in 0..n) {
        print("{} number from fibonacci sequence is {}", i, next_term);
        t1 = t2;
        t2 = next_term;
        next_term = t1 + t2;
    }
}

fun main(): void {
    print("How many numbers should i generate?");
    var n = parse_i32(input());

    print_fibonacci(n);
}