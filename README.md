# read-ctags-rs

`read-ctags-rs` is a package for efficiently reading a file generated by
`ctags` (e.g. [universal ctags](https://ctags.io)  and extracting all unique
tokens, stripping out any unnecessary metadata in the process.

This is based off of a similar tool I wrote in Haskell,
[`read-ctags`](https://github.com/joshuaclayton/read-ctags).

## Install from Source

```sh
cargo install --path .
```

## Test

```sh
cargo test
```

## License

Copyright 2020 Josh Clayton. See the [LICENSE](LICENSE).
