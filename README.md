# primitive-types

`primitive-types` adds several extension traits, that make it easy to read primitive types from any type that implements `std::io::Read`

## Examples

```rust

    fn main() {
        let bytes: [u8; 8] = [24, 45, 68, 84, 251, 33, 9, 64];
        let float = bytes.as_slice().read_le_f64().unwrap();
        printf!("{float}");
    }

```
> 3.141592653589793