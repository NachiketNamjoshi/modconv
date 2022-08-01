# Mode Converter

Converts linux file modes to readable format and vice versa.

# Usage
## Absolute Conversion
```
$ modconv absolute 777
Absolute Value :: 777
Symbolic Value :: rwxrwxrwx
```

## Symbolic Conversion
```
$ modconv symbolic rwx------
Absolute Value :: 700
Symbolic Value :: rwx------
```

# Building
You need to install cargo/rust. Please install it via your distribution's package manager or through [rustup](https://rustup.rs/)

```
cargo build --release
```
