# Rust Arg Parsing Benchmarks

This repo tries to assess Rust arg parsing performance.

We currently compare:

Name                                                 | Style                 | Notes
-----------------------------------------------------|-----------------------|------
No-op                                                | N/A                   | N/A
**[argp](https://github.com/jirutka/argp)**          | `derive`              | Added in this fork
[argh](https://github.com/google/argh)               | `derive`              |
[bpaf](https://github.com/pacak/bpaf)                | Combinatoric or `derive` |
[clap_lex](https://github.com/clap-rs/clap)          | Imperative            | No help generation
[clap](https://github.com/clap-rs/clap)              | Builder or `derive`   | Color, suggested fixes, completions
[gumdrop](https://github.com/murarth/gumdrop)        | `derive`              |
[lexopt](https://github.com/blyxxyz/lexopt)          | Imperative            | No help generation
[pico-args](https://github.com/razrfalcon/pico-args) | Imperative            | No help generation
[xflags](https://github.com/matklad/xflags)          | proc-macro            |

See also [an examination of design trade offs](docs/tradeoffs.md)

*Note: any non-performance comparison is meant to provide context for what you
gain/lose with each crate's overhead.  For a full comparison, see each parser
docs*

# Results

Name | Overhead (release) | Build (debug) | Parse (release) | Invalid UTF-8 | Downloads | Version
-----|--------------------|---------------|-----------------|---------------|-----------|--------
null | 0 KiB | 639ms *(full)* <br/>418ms *(incremental)* | 1ms | Y | - | -
**argp** | **36 KiB** | 7s *(full)* <br/>554ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/argp) | v0.2.0
argh | 24 KiB | 7s *(full)* <br/>550ms *(incremental)* | 2ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.10
bpaf | 124 KiB | 2s *(full)* <br/>588ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
bpaf_derive | 124 KiB | 11s *(full)* <br/>607ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
clap | 348 KiB | 9s *(full)* <br/>930ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap-minimal | 220 KiB | 5s *(full)* <br/>766ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_derive | 364 KiB | 16s *(full)* <br/>1s *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_lex | 16 KiB | 1s *(full)* <br/>530ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.4.1
gumdrop | 20 KiB | 7s *(full)* <br/>538ms *(incremental)* | 2ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 20 KiB | 955ms *(full)* <br/>465ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 16 KiB | 976ms *(full)* <br/>468ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 16 KiB | 2s *(full)* <br/>443ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

*System: Linux 5.15.0-1034-azure (x86_64) w/ `-j 2` (GitHub actions, [see log](https://github.com/jirutka/argparse-rosetta-rs/actions/runs/4737610982/jobs/8410591911))*

*rustc: rustc 1.68.2 (9eb3afe9e 2023-03-27)*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.
- Size is measured on release builds with `panic = "abort"` and `strip = true`.

# Running the Benchmarks

```bash
$ ./bench.py
$ ./format.py
```

To be included, the crate needs meet one of the following criteria:
- 10k+ recent downloads
- Unique API design

# Special Thanks

- RazrFalcon for creating the [initial benchmarks](https://github.com/RazrFalcon/pico-args)
- djc for inspiration with [template-benchmarks-rs](https://github.com/djc/template-benchmarks-rs)
- sharkdp for [hyperfine](https://github.com/sharkdp/hyperfine)
