# Mars weight calculator ()

This is sample command line program based on [Udemy Rust training](https://www.udemy.com/course/rust-fundamentals/learn/lecture/20695560#overview).

The instructor has all sample codes in the [github repo](https://github.com/gavadinov/Learn-Rust-by-Building-Real-Applications).

## Learnings

For reading use input we use [std:io package](https://doc.rust-lang.org/std/io/index.html)

### Ownership

- Each value is owned by a variable

```rs
let x = 5;
```

- When the owner goes out of the scope the value is deallocated

- There can only be ONE owner of variable/value at the time

### Unwrap

Is used when fn/method returns Result. It will unwrap the Result into value or throw an error (panic). Unwrap is basical approach of unwraping the Result struct. Other methods are match Result{}, or if_err() etc.
