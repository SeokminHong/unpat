# Unpattern

Unbox and declare a variables from the pattern with easy syntax.

Sometimes, it is needed to unbox an enum even you know what the enum exactly matches,
however you must use `match` or `if let` expression to unbox it.

For example:

```rust
enum TestEnum {
    Int(i32),
    Tuple(i32 i32),
}

let test = TestEnum::Int(3);
// You need to unbox the enum.
if let TestEnum::Int(v) = test {
    assert_eq!(v, 3);
} else {
    unreachable!();
}
```

This crate helps to unbox the enum from a pattern.

```rust
let test = TestEnum::Int(3);
unpat!(TestEnum::Int(v) <- test); // `v = 3` binding is created here.
assert_eq!(v, 3);
```

The concept of the crate is inspired from Elixir's pattern matching.

```elixir
test = %Test{a: 1, b: 2}
%Test{a} = test # `a = 1` binding is created here
```

## Usage

```rust
struct TestStruct {
    int: i32,
    tuple: (i32, i32),
}

enum TestEnum {
    Int(i32),
    Tuple(i32, f64),
    Struct(TestStruct),
}

let test = TestEnum::Struct(TestStruct {
    int: 1,
    tuple: (2, 3),
});

unpat!(
    TestEnum::Struct(
        TestStruct { int, tuple: (x, y) }
    ) <- test
);
assert_eq!((int, x, y), (1, 2, 3));
```

Also, the named field can be bound with the `@` syntax.

```rust
unpat!(
    TestEnum::Struct(
        TestStruct { int, tuple: v @ (x, y) }
    ) <- test
);
assert_eq!((int, x, y), (1, 2, 3));
assert_eq!(v, (2, 3));
```

## Error Handling

`unpat` panics if the pattern doesn't match. If you want to handle the error, use `try_unpat` instead.

```rust
let test = TestEnum::Int(3);
try_unpat!(TestEnum::Int(v) <- test, String::from("The pattern doesn't match"));
assert_eq!(v, 3);

// The pattern doesn't match. It will return Err.
try_unpat!(TestEnum::Tuple(a, _) <- test, String::from("The pattern doesn't match"));
unreachable!()
```

## TODO

- [ ] Implement all patterns
- [ ] Caret syntax

## Related

[enum-as-inner](https://github.com/bluejekyll/enum-as-inner)
