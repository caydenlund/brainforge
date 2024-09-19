# BrainForge

A compiler for the [brainfuck language](https://github.com/sunjay/brainfuck/blob/master/brainfuck.md).


## Usage


### `bfc`

Compiles the given BF program, either given a filename or receiving input piped from stdin.
The output is saved in, by default, `./a.s`, but can be controlled with the `-o` option.

Available options:

```
  -o, --output <OUTPUT>    The output file [default: a.s]
  -m, --memsize <MEMSIZE>  The size of the memory tape [default: 4096]
  -h, --help               Print help
  -V, --version            Print version
```

Examples:

```bash
$  bfc prgm.bf -m 8192 -o prgm.s
$  bfc < prgm.bf | nvim
```


### `bf-interp`

Interprets the given BF program, either given a filename or receiving input piped from stdin.

Available options:

```
  -p, --profile            Whether to profile the given program
  -m, --memsize <MEMSIZE>  The size of the memory tape [default: 4096]
  -h, --help               Print help
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
