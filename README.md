# rust-examples

## Run it

```console
$ cargo run                  # every example, in order
$ cargo run -- 14            # section 14, Lifetimes
$ cargo run -- 14.6          # one particular example
$ cargo run -- closures      # sections whose title contains "closures"
$ cargo run -- --list        # table of contents
```

Each slide prints its number, title and caption, then runs:

## Test it

```console
$ cargo test
```

## Files

```text
src/
  lib.rs            Section, Example and Kind: the shape of the catalog
  catalog.rs        all 146 examples in order, with titles and captions
  main.rs           the runner behind `cargo run`
  sections/
    mod.rs          lint settings for slide code (see below)
    s01_variable_bindings.rs
    ...
    s21_returning_closures.rs
does_not_compile/   one program that must fail
tests/catalog.rs    the checks described above
```

## Notes

Requires Rust 1.85 or newer (edition 2024).

## Credits

The structure and topics follow
[*A half-hour to learn Rust*](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
by Amos Wenger. The examples are the deck's own rewrites of the article's ideas.
