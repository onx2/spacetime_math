# spacetimedb_math

Common math types and SpacetimeDB integrations for server-side Rust modules.

## Features

### Scalar selection
- `f32` (default) — use `f32` as `Scalar`.
- `f64` — use `f64` as `Scalar`.

> `f32` and `f64` are mutually exclusive.

### Optional integrations
- `serde` — enable `Serialize`/`Deserialize` derives.
- `glam` — enable `From` conversions with `glam` types.
- `nalgebra` — enable `From` conversions with `nalgebra` types.

You can enable both `glam` and `nalgebra` at the same time.

## Usage

Add the crate with your desired features:

```
[dependencies]
spacetimedb_math = { version = "0.1", features = ["f32", "glam", "serde"] }
```

## License

MIT
