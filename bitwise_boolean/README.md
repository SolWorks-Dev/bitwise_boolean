# bitwise_boolean

A Rust macro to generate bitwise boolean getters and setters for a struct.

## Usage

To use the `bitwise_boolean` macro, add the following to your `Cargo.toml` file:

```toml
[dependencies]
bitwise_boolean = "0.1.0"
```

Then, use the macro in your code:

```rust
use bitwise_boolean::BitwiseBoolean;

#[derive(Default, BitwiseBoolean)]
struct ExampleSettingsData {
    // up to 8 flags total
    #[bitwise_bool(
        admin,
        agreed_to_terms,
        banned,
        random_other_flag
    )]
    pub booleans: u8,
}

struct ExampleSettings {
    data: ExampleSettingsData,
}

impl ExampleSettings {
    fn new() -> Self {
        Self {
            data: ExampleSettingsData::default(),
        }
    }
}
```

Now, you can use the bitwise boolean getters and setters in your code:

```rust
let mut settings = ExampleSettings::new();

settings.data.set_admin(true);
assert!(settings.data.admin());
assert!(!settings.data.agreed_to_terms());
assert!(!settings.data.banned());
assert!(!settings.data.random_other_flag());

settings.data.set_agreed_to_terms(true);
assert!(settings.data.admin());
assert!(settings.data.agreed_to_terms());
assert!(!settings.data.banned());
assert!(!settings.data.random_other_flag());

settings.data.set_banned(true);
assert!(settings.data.admin());
assert!(settings.data.agreed_to_terms());
assert!(settings.data.banned());
assert!(!settings.data.random_other_flag());

settings.data.set_random_other_flag(true);
assert!(settings.data.admin());
assert!(settings.data.agreed_to_terms());
assert!(settings.data.banned());
assert!(settings.data.random_other_flag());
```

## Limitations

The `bitwise_boolean` macro currently supports up to 8 flags per struct.

## License

This project is licensed under the MIT License.