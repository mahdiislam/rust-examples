//! Every example from the "A half-hour to learn Rust" slide deck, one entry per slide.
//!
//! * [`catalog::SECTIONS`] lists the 21 sections and 146 slides in deck order.
//! * `src/sections/` holds the code of every slide that compiles, one function per slide.
//! * `does_not_compile/` holds the slides that are meant to be compile errors,
//!   as complete programs. `cargo test` checks that rustc rejects each one
//!   with the error code the slide is about.

pub mod catalog;
pub mod sections;

pub use catalog::SECTIONS;

/// One section of the deck, such as "07 Patterns".
pub struct Section {
    pub number: usize,
    pub title: &'static str,
    pub examples: &'static [Example],
}

/// One slide of the deck.
pub struct Example {
    pub title: &'static str,
    pub caption: &'static str,
    pub kind: Kind,
}

/// What a slide's code does when you try it.
pub enum Kind {
    /// Compiles and runs to the end.
    Runs(fn()),
    /// Compiles, and panics when run. That panic is what the slide shows.
    Panics(fn()),
    /// A complete program that rustc must reject with exactly this error code.
    DoesNotCompile {
        file: &'static str,
        source: &'static str,
        error: &'static str,
    },
    /// A slide with bullet points and no code.
    Notes(&'static [&'static str]),
}

impl Section {
    /// The slide number as shown by the runner: "07.3" is the third slide of section 7.
    pub fn slide_id(&self, index: usize) -> String {
        format!("{:02}.{}", self.number, index + 1)
    }
}

impl Kind {
    pub fn label(&self) -> &'static str {
        match self {
            Kind::Runs(_) => "runs",
            Kind::Panics(_) => "panics",
            Kind::DoesNotCompile { .. } => "does not compile",
            Kind::Notes(_) => "notes",
        }
    }
}
