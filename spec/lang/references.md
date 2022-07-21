### References

Reference is like an alias to a value in the memory, you can use value that is referenced be the reference with the same syntax as from a normal value.

Reference is like a pointer, that allows us to use the value without taking it's ownership. It is destroyed when the value that is referenced is destroyed.

### Ownership

Every value that is in a scope by default has an owner. Owner is a scope that this variable is in. Variable is automatically removed from the memory when the owner dies, for example

```
{ // a is invalid here as it is not defined yet
    var a = 1; // from this point a is valid as it is defined
    ... // a is still valid here
} // from this point a is invalid as it's owner scope has ended
```
