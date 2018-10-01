# Envoy

Envoy is a small library providing more convenient abstractions for manipulating environment variables than the standard library. In particular, Envoy has APIs for manipulating `PATH`-like variables.

## Example

```rust
let path = envoy::path(); // get the `PATH`/`Path` variable
let updated = path.split()
    .remove(&"/usr/local/bin")
    .prefix(&"/home/dherman/.bin");
envoy::set_path(updated.join().unwrap());
```

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Code of Conduct

Contribution to Envoy is organized under the terms of the [Contributor Covenant Code of Conduct](https://github.com/notion-cli/notion/blob/master/CODE_OF_CONDUCT.md).
The maintainer of Envoy, Dave Herman, personally promises to work actively to uphold that code of conduct.
We aim to foster a community that is welcoming, inclusive, empathetic, and kind.
If you share those goals and want to have fun hacking environment variables, we invite you to join us!
