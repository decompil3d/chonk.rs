# `chonk-cli`

Check the size of a file and determine its chonkiness

Inspired by <https://knowyourmeme.com/memes/chonk-oh-lawd-he-comin>

Ported from the Javascript <https://github.com/decompil3d/chonk>

## Installation

```sh
# Local
cargo install chonk-cli
```

## Usage

```sh
chonk-cli [OPTIONS] [PATHS]...
```

### Options

```sh-session
Options:
  -c, --chonk <CHONK>    Minimum size in kilobytes for It chonk [default: 50]
    -e, --heck <HECK>      Minimum size in kilobytes for A heckin' chonker [default: 500]
    -h, --help             Print help information
    -H, --hefty <HEFTY>    Minimum size in kilobytes for HEFTYCHONK [default: 2000]
    -L, --lawd <LAWD>      Minimum size in kilobytes for OH LAWD, IT COMIN'! [default: 50000]
    -M, --mega <MEGA>      Minimum size in kilobytes for MEGACHONKER [default: 10000]
```

### Sample

```sh-session
$ chonk-cli lawd.dmg mega.zip hefty.tar.gz heckin.jpg chonk.pdf fine.txt
```

![sample output](chonk.png)
