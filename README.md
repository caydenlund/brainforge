# BrainForge

A compiler for the [brainfuck language](https://github.com/sunjay/brainfuck/blob/master/brainfuck.md).

## Usage

### `bfc`

Compiles the given BF program, either given a filename or receiving input piped from stdin.
The output is saved in, by default, `./a.s`, but can be controlled with the `-o` option.

Available options:

```
Usage: bfc [OPTIONS] [FILE]

Arguments:
  [FILE]
          The file to run
          
          If one is not provided, then reads a program from stdin

Options:
  -o, --output <OUTPUT>
          The output file
          
          Use `-` for stdout
          
          [default: a.s]

  -m, --memsize <MEMSIZE>
          The size of the memory tape
          
          [default: 8192]

  -l, --loops
          Whether to perform simple loop flattening

  -s, --scan
          Whether to perform memory scan vectorization

  -p, --partial-evaluation
          Whether to perform partial evaluation

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Examples:

```bash
$  bfc prgm.bf -m 8192 -o prgm.s
$  bfc < prgm.bf | nvim
```

### `bf-interp`

Interprets the given BF program, either given a filename or receiving input piped from stdin.

```
Usage: bf-interp [OPTIONS] [FILE]

Arguments:
  [FILE]
          The file to run
          
          If one is not provided, then reads a program from stdin

Options:
  -p, --profile
          Whether to profile the given program

  -m, --memsize <MEMSIZE>
          The size of the memory tape
          
          [default: 8192]

  -l, --loops
          Whether to perform simple loop flattening

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

Examples:

```bash
$  bf-interp prgm.bf -m 8192
$  bf-interp -p < prgm.bf
```

## `bf-jit`

Compiles the given program just-in-time and executes it.
Provide it with a command-line argument filename, or pipe it a program through stdin.

```
Usage: bf-jit [OPTIONS] [FILE]

Arguments:
  [FILE]
          The file to run
          
          If one is not provided, then reads a program from stdin

Options:
  -m, --memsize <MEMSIZE>
          The size of the memory tape
          
          [default: 8192]

  -l, --loops
          Whether to perform simple loop flattening

  -s, --scan
          Whether to perform memory scan vectorization

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Building

Compile with Cargo.
The binary executable is put in the `target/[profile]` directory.

```bash
$  cargo build --release
$  ./target/release/bf-interp  # [filename] [options...]
```
