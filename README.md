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
null | 0 KiB | 475ms *(full)* <br/>328ms *(incremental)* | 1ms | Y | - | -
**argp** | **44 KiB** | 8s *(full)* <br/>436ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/argp) | v0.3.0
argh | 24 KiB | 6s *(full)* <br/>420ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.10
bpaf | 124 KiB | 2s *(full)* <br/>469ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
bpaf_derive | 124 KiB | 8s *(full)* <br/>457ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.10
clap | 348 KiB | 7s *(full)* <br/>746ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap-minimal | 220 KiB | 4s *(full)* <br/>595ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_derive | 364 KiB | 12s *(full)* <br/>828ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.2.0
clap_lex | 16 KiB | 785ms *(full)* <br/>394ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.4.1
gumdrop | 20 KiB | 6s *(full)* <br/>422ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 20 KiB | 755ms *(full)* <br/>354ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 16 KiB | 732ms *(full)* <br/>363ms *(incremental)* | 6ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 16 KiB | 1s *(full)* <br/>350ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

*System: Linux 5.15.0-1034-azure (x86_64) w/ `-j 2` (GitHub actions, [see log](https://github.com/jirutka/argparse-rosetta-rs/actions/runs/4748657006/jobs/8435104154))*

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
