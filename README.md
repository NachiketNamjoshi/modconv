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

## Caveats
1. To convert Symbolic notation that starts with "--" will need to be passed in like so:
	```
	$ modconv symbolic -- --x--x--x
	Absolute Value :: 111
	Symbolic Value :: --x--x--x
	```

# Building
You need to install cargo/rust. Please install it via your distribution's package manager or through [rustup](https://rustup.rs/)

```
cargo build --release
```
