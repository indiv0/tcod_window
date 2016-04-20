# tcod_window

<table>
    <tr>
        <td><strong>Linux / OS X</strong></td>
        <td><a href="https://travis-ci.org/indiv0/tcod_window" title="Travis Build Status"><img src="https://travis-ci.org/indiv0/tcod_window.svg?branch=master" alt="travis-badge"></img></a></td>
    </tr>
    <tr>
        <td colspan="2">
            <a href="https://indiv0.github.io/tcod_window/tcod_window" title="API Docs"><img src="https://img.shields.io/badge/API-docs-blue.svg" alt="api-docs-badge"></img></a>
            <a href="https://crates.io/crates/tcod_window" title="Crates.io"><img src="https://img.shields.io/crates/v/tcod_window.svg" alt="crates-io"></img></a>
            <a href="#License" title="License: MIT/Apache-2.0"><img src="https://img.shields.io/crates/l/tcod_window.svg" alt="license-badge"></img></a>
            <a href="https://coveralls.io/github/indiv0/tcod_window?branch=master" title="Coverage Status"><img src="https://coveralls.io/repos/github/indiv0/tcod_window/badge.svg?branch=master" alt="coveralls-badge"></img></a>
            <a href="http://clippy.bashy.io/github/indiv0/tcod_window/master/log" title="Clippy Linting Result"><img src="http://clippy.bashy.io/github/indiv0/tcod_window/master/badge.svg" alt="clippy-lint-badge"></img></a>
        </td>
    </tr>
</table>

A TCOD back-end for the Piston game engine.


# Table of Contents

* [Usage](#usage)
* [Contributing](#contributing)
* [Credits](#credits)
* [License](#license)

## Usage

Add the following to your `Cargo.toml`:

```toml
[dependencies]
tcod_window = "0.2"
```

And in your `lib.rs` or `main.rs`:

```rust
extern crate tcod_window;
```

See the [API docs][api-docs] for information on using the crate in your library.

For usage examples, please the see the [examples][examples] directory of the
project.

## Contributing

Contributions are always welcome!
If you have an idea for something to add (code, documentation, tests, examples,
etc.) feel free to give it a shot.

Please read [CONTRIBUTING.md][contributing] before you start contributing.

## Credits

The list of contributors to this project can be found at
[CONTRIBUTORS.md][contributors].

## License

`tcod_window` is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE][license-apache], and [LICENSE-MIT][license-mit] for details.

[api-docs]: https://indiv0.github.io/tcod_window/tcod_window
[contributing]: https://github.com/indiv0/tcod_window/blob/master/CONTRIBUTING.md "Contribution Guide"
[contributors]: https://github.com/indiv0/tcod_window/blob/master/CONTRIBUTORS.md "List of Contributors"
[examples]: https://github.com/indiv0/tcod_window/tree/master/examples "tcod_window - Examples"
[license-apache]: https://github.com/indiv0/tcod_window/blob/master/LICENSE-APACHE "Apache-2.0 License"
[license-mit]: https://github.com/indiv0/tcod_window/blob/master/LICENSE-MIT "MIT License"
