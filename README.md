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
null | 0 KiB | 477ms *(full)* <br/>325ms *(incremental)* | 1ms | Y | - | -
**argp** | **57 KiB** | 6s *(full)* <br/>412ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/argp) | v0.2.0
argh | 39 KiB | 6s *(full)* <br/>409ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/argh) | v0.1.9
bpaf | 215 KiB | 2s *(full)* <br/>471ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.7
bpaf_derive | 214 KiB | 9s *(full)* <br/>464ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/bpaf) | v0.7.7
clap | 608 KiB | 7s *(full)* <br/>745ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.4
clap-minimal | 428 KiB | 4s *(full)* <br/>614ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.4
clap_derive | 645 KiB | 15s *(full)* <br/>830ms *(incremental)* | 2ms | Y | ![Download count](https://img.shields.io/crates/dr/clap) | v4.1.4
clap_lex | 37 KiB | 950ms *(full)* <br/>401ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/clap_lex) | v0.3.1
gumdrop | 37 KiB | 5s *(full)* <br/>426ms *(incremental)* | 1ms | N | ![Download count](https://img.shields.io/crates/dr/gumdrop) | v0.8.1
lexopt | 36 KiB | 750ms *(full)* <br/>358ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/lexopt) | v0.3.0
pico-args | 32 KiB | 733ms *(full)* <br/>364ms *(incremental)* | 6ms | Y | ![Download count](https://img.shields.io/crates/dr/pico-args) | v0.5.0
xflags | 27 KiB | 1s *(full)* <br/>355ms *(incremental)* | 1ms | Y | ![Download count](https://img.shields.io/crates/dr/xflags) | v0.3.1

*System: Linux 5.15.0-1034-azure (x86_64) w/ `-j 2` (GitHub actions, [see log](https://github.com/jirutka/argparse-rosetta-rs/actions/runs/4527123109/jobs/7972720383))*

*rustc: rustc 1.68.1 (8460ca823 2023-03-20)*

Notes:
- Overhead will be lower if your application shares dependencies with your argument parsing library.

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
