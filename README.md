# npm-name

Check whether a package name is available on npmjs.com.

- [Documentation](https://docs.rs/npm-name)

## Usage
```sh
npm-name 0.1.0
Weslei Juan Moser Pereira <wesleimsr@gmail.com>
Check whether a package name is available on npmjs.com

USAGE:
    npm-name <name>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <name>    The name of the package you're targeting.
```

## Installation

There are several ways to install `npm-name`. Choose a more convenient option for you.

### Pre-compiled binary

```sh
# Linux / macOS / Windows (MINGW and etc). Installs it into ./bin/ by default
$ curl -sSfL https://raw.githubusercontent.com/wesleimp/npm-name/master/scripts/install.sh | sh -s

# Specify installation directory and version
$ curl -sSfL https://raw.githubusercontent.com/wesleimp/npm-name/master/scripts/install.sh | sh -s -- -b usr/local/bin v0.1.0

# Alpine Linux (wget)
$ wget -q -O - https://raw.githubusercontent.com/wesleimp/npm-name/master/scripts/install.sh | sh -s
```

### Cargo

If you are a Rust programmer, you can install `npm-name` via `cargo`:

```sh
$ cargo install npm-name
```

## License
[MIT](./LICENSE)
