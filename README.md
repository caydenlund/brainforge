# BrainForge

An compiler for the [brainfuck language](https://github.com/sunjay/brainfuck/blob/master/brainfuck.md).


## Usage

### `bf-interp`

Runs the given BF program, either given a filename or receiving input piped from stdin.

Available options:

```
  -p, --profile            Whether to profile the given program
  -m, --memsize <MEMSIZE>  The size of the memory tape [default: 4096]
  -h, --help               Print help (see more with '--help')
  -V, --version            Print version
```

Examples:

```bash
$  bf-interp prgm.bf -m 8192
$  bf-interp -p < prgm.bf
```


## Building

Compile with Cargo.
The binary executable is put in the `target/[profile]` directory.

```bash
$  cargo build --release
$  ./target/release/bf-interp  # [filename] [options...]
```
