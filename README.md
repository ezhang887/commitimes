# commitimes

![status](https://github.com/ezhang887/commitimes/actions/workflows/build.yaml/badge.svg)
![status](https://github.com/ezhang887/commitimes/actions/workflows/binary.yaml/badge.svg)

Plot a histogram of a git repo's commit times in the terminal:
```
~/Github/commitimes master
❯ commitimes
     2-|   -----
       |   | | |
       |   | | |
   1.6-|   | | |
       |   | | |
       |   | | |
   1.2-|   | | |
       |   | | |----                      ---        ---   ---
       |   | | | | |                      | |        | |   | |
   0.8-|   | | | | |                      | |        | |   | |
       |   | | | | |                      | |        | |   | |
       |   | | | | |                      | |        | |   | |
   0.4-|   | | | | |                      | |        | |   | |
       |   | | | | |                      | |        | |   | |
       |   | | | | |                      | |        | |   | |
     0+------------------------------------------------------------
      |         |          |         |          |         |
      0         4          8        12         16        20

```

### Installation

A binary is availible to download on the [release page](https://github.com/ezhang887/commitimes/releases/tag/release).

Alternatively, this can be built from source (clone the repo and run `cargo build --release`).
