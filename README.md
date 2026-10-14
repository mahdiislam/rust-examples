# rust-examples

Every example from the **"A half-hour to learn Rust"** slide deck (`rust-half-hour.typ`),
as one Cargo project you can run and test. It has no dependencies and needs no network.

The deck has 21 sections and 146 slides. Each slide is one entry here:

| Slides | What they are | Where the code lives | What `cargo test` checks |
| --- | --- | --- | --- |
| 122 | Code that compiles and runs | One function per slide in `src/sections/` | It runs, and every value the slide's comments claim is asserted |
| 5 | Code that panics on purpose | Same, marked as panicking | It really panics |
| 17 | Code that must **not** compile | One complete program per slide in `does_not_compile/` | rustc rejects it, with exactly the error code the slide is about |
| 2 | Bullet points, no code | Text in `src/catalog.rs` | (nothing to run) |

## Run it

```console
$ cargo run                  # every slide, in order
$ cargo run -- 14            # section 14, Lifetimes
$ cargo run -- 14.6          # one slide
$ cargo run -- closures      # sections whose title contains "closures"
$ cargo run -- --list        # table of contents
```

Each slide prints its number, title and caption, then runs:

```text
12.3  panic!  [panics]
Stops the thread with a message and its location.
panicked: Something broke at src/sections/s12_macros_and_panics.rs:30

14.6  Shared XOR mutable  [does not compile]
A value can't be mutably borrowed while immutably borrowed.
rustc rejects this program with error[E0502] (does_not_compile/14_06_shared_xor_mutable.rs):
    fn main() {
        let mut n = 5;
        ...
```

## Test it

```console
$ cargo test
```

The tests in `tests/catalog.rs`:

- run every runnable slide, so every `assert!` in its checks must hold;
- run every panicking slide and make sure it panics;
- compile every program in `does_not_compile/` with `rustc --edition 2024`. Each must fail,
  and the set of error codes rustc reports must be exactly the expected one (E0382, E0597, ...).
  This test needs `rustc` on your `PATH`, which rustup provides.

## How a slide becomes code

The slide's code is copied in unchanged. A few marked lines make it run:

```rust
pub fn borrowing_with_mut() {
    // ---- setup ----
    struct Temp { freezing: bool, degrees: i32 }      // defined on an earlier slide
    fn show_temp(t: &Temp) { println!("{}°", t.degrees); }

    // ---- slide ----
    fn warm_up(t: &mut Temp) {
        t.degrees += 10;
    }

    fn main() { ... }                                 // exactly as on the slide

    // ---- check ----
    main();
    let mut t = Temp { freezing: true, degrees: -4 };
    warm_up(&mut t);
    assert_eq!(t.degrees, 6);
}
```

- **setup**: definitions the slide relies on from earlier slides, or helpers the slide
  only names (like `report()` or `load_config()`).
- **slide**: the slide's code. Rust allows structs, functions, traits and `impl`s
  inside a function body, so every slide keeps its own `Temp`, its own `fn main`, and so on,
  and slides never clash.
- **check**: calls the slide's `main`, prints the results, and asserts what the slide's
  comments say, such as `// prints 5` or `// "alloc::vec::Vec<i32>"`.

Where a slide shows two versions of the same function side by side (for example *Implicit
return* or *Lifetime elision*), each version sits in its own `{ }` block, because two items
with the same name can't share a scope.

## Files

```text
src/
  lib.rs            Section, Example and Kind: the shape of the catalog
  catalog.rs        all 146 slides in deck order, with titles and captions
  main.rs           the runner behind `cargo run`
  sections/
    mod.rs          lint settings for slide code (see below)
    s01_variable_bindings.rs
    ...
    s21_returning_closures.rs
does_not_compile/   one program per slide that must fail, named by slide number
tests/catalog.rs    the checks described above
```

## Notes

- **Lints.** Slides are fragments written for reading. They leave values unused, write
  `return 7;` and `Pet { name: name }`, call `.unwrap()` on `Some`, and so on. Those lints are
  allowed in `src/sections/mod.rs` only. The rest of the project builds with no rustc or
  clippy warnings.
- **Formatting.** `rustfmt` skips `src/sections/` so the code keeps the layout it has on the
  slides, including aligned comments. Everything else is formatted with `cargo fmt`.
- **One wording difference.** For slide 01.5, current rustc says `used binding `total` is
  possibly-uninitialized`. The slide's comment says "isn't initialized". The error code,
  E0381, is the same.
- **Type names.** Slide 11.7 prints `my_crate::Both<f64>`. Here the full path starts with
  this crate's module path, so the check only looks at the `::Both<f64>` ending.

Requires Rust 1.85 or newer (edition 2024).

## Credits

The structure and topics follow
[*A half-hour to learn Rust*](https://fasterthanli.me/articles/a-half-hour-to-learn-rust)
by Amos Wenger. The examples are the deck's own rewrites of the article's ideas.
