# xee-xpath
[![Crates.io](https://img.shields.io/crates/v/xee-xpath.svg)](https://crates.io/crates/xee-xpath)
[![Documentation](https://docs.rs/xee-xpath/badge.svg)](https://docs.rs/xee-xpath)


This provides a high-level XPath API to use to make queries programmatically
from Rust. It implements [XPath 3.1](https://www.w3.org/TR/xpath-31/) including
most of its [standard library of
functions](https://www.w3.org/TR/xpath-functions-31/).

The [API docs](https://docs.rs/xee-xpath/latest/xee_xpath/) contain a usage
example.

This is top-level API crate of the [Xee
project](https://github.com/Paligo/xee). For the
[`xee`](https://github.com/Paligo/xee/xee) commandline tool built with
`xee-xpath`, download a [release](https://github.com/Paligo/xee/releases/) or
[build it
yourself](https://github.com/Paligo/xee?tab=readme-ov-file#obtaining-the-xee-commandline-tool).

## More Xee

[Xee homepage](https://github.com/Paligo/xee)

## Credits

This project was made possible by the generous support of
[Paligo](https://paligo.net/).

## Benchmarks

There are a few tiny benchmarks at the time of writing.

Run benchmarks like this:

```
cargo bench --benches
```