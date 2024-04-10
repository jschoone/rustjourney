# Chapter 02

Regarding
```rust
io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```

Since `read_line` returns a Result. Result is an `enumeration` in short `enum`, which looks like this:

```rust
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

`enums` have different variants, for `Result` those are `Ok` and `Err`. `T` and `E` are generics which will be covered later.
The compiler checks if all variants from an `enum` are handled. We do this here by using the `expect`.
If `read_line` is successful it returns the value of `T` from `Ok(T)` which is is a `String` in this case.
If `read_line` errors, `expect` prints *Failed to read line*

