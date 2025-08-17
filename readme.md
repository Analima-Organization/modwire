# modwire
```toml
[dependencies]
modwire = { version = "*" }
```

# Tools for module organisation.
###### Example: `expose!`
```rust
// /foo.rs
// /bar.rs
// /foo_bar.rs
// lib.rs
::modwire::expose!(
    pub foo
    pub bar
        foo_bar
);
```