import std::io { input, print, println };
import std::conversion { parse_i32 };
import std::math;

fun main(): void {
    print("Enter number a: ");
    var a = parse_i64(input());

    print("Enter operation (+, -, *, /, ^):");
    var op = input()[0];
    
    print("Enter number b: ");
    var b = parse_i64(input());

    var result = match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        '/' => a / b,
        '^' => math::pow(a, b)
    }

    print("Result of {a} {op} {b} is {result}", {
        a: a,
        op: op,
        b: b,
        result: result
    });
}