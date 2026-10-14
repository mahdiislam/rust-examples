//! Walks through the deck: runs each slide's code and shows what happens.
//!
//!     cargo run                 every slide, in order
//!     cargo run -- 14           one section
//!     cargo run -- 14.3         one slide
//!     cargo run -- closures     sections whose title contains a word
//!     cargo run -- --list       the table of contents

use std::env;
use std::io::IsTerminal;
use std::panic::{self, AssertUnwindSafe};
use std::process::ExitCode;
use std::sync::OnceLock;

use rust_examples::{Example, Kind, SECTIONS, Section};

const USAGE: &str = "\
Usage: rust-examples [SELECTION | --list | --help]

With no arguments, every slide runs in order. SELECTION can be:
  14          one section, by number
  14.3        one slide, by section and slide number
  closures    every section whose title contains this text
";

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    let selection = match args.as_slice() {
        [] => Selection::Everything,
        [flag] if flag == "-h" || flag == "--help" => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        [flag] if flag == "-l" || flag == "--list" => {
            print_contents();
            return ExitCode::SUCCESS;
        }
        [query] => match Selection::parse(query) {
            Some(selection) => selection,
            None => {
                eprintln!("Nothing matches '{query}'. Try --list to see every section.");
                return ExitCode::FAILURE;
            }
        },
        _ => {
            eprint!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    // Panics are part of the show: print them as a one-line note under the slide.
    panic::set_hook(Box::new(|info| {
        let payload = info.payload();
        let message = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
            .unwrap_or("(no message)");
        let place = info
            .location()
            .map(|l| format!(" at {}:{}", l.file(), l.line()))
            .unwrap_or_default();
        println!("{}", paint(RED, &format!("panicked: {message}{place}")));
    }));

    let mut surprises = Vec::new();
    for section in SECTIONS {
        let slides: Vec<usize> = (0..section.examples.len())
            .filter(|&i| selection.includes(section, i))
            .collect();
        if slides.is_empty() {
            continue;
        }
        println!(
            "\n{}",
            paint(BOLD, &format!("§ {:02}  {}", section.number, section.title))
        );
        for i in slides {
            if let Some(surprise) = show(section, i, &section.examples[i]) {
                surprises.push(surprise);
            }
        }
    }

    if surprises.is_empty() {
        ExitCode::SUCCESS
    } else {
        eprintln!("\nThese slides did not behave as expected:");
        for surprise in &surprises {
            eprintln!("  {surprise}");
        }
        ExitCode::FAILURE
    }
}

/// Print one slide and run its code. Returns a description if it misbehaved.
fn show(section: &Section, index: usize, example: &Example) -> Option<String> {
    let id = section.slide_id(index);
    println!(
        "\n{}  {}  {}",
        paint(DIM, &id),
        paint(BOLD, example.title),
        paint(DIM, &format!("[{}]", example.kind.label()))
    );
    if !example.caption.is_empty() {
        println!("{}", paint(DIM, example.caption));
    }

    match &example.kind {
        Kind::Runs(run) => {
            let result = panic::catch_unwind(AssertUnwindSafe(run));
            result
                .is_err()
                .then(|| format!("{id} {} panicked, but should run cleanly", example.title))
        }
        Kind::Panics(run) => {
            let result = panic::catch_unwind(AssertUnwindSafe(run));
            result
                .is_ok()
                .then(|| format!("{id} {} should have panicked", example.title))
        }
        Kind::DoesNotCompile {
            file,
            source,
            error,
        } => {
            println!(
                "{}",
                paint(
                    YELLOW,
                    &format!("rustc rejects this program with error[{error}] ({file}):")
                )
            );
            for line in source
                .lines()
                .skip_while(|line| line.starts_with("//") || line.is_empty())
            {
                println!("    {line}");
            }
            None
        }
        Kind::Notes(points) => {
            for point in *points {
                println!("  - {point}");
            }
            None
        }
    }
}

fn print_contents() {
    for section in SECTIONS {
        println!("\n§ {:02}  {}", section.number, section.title);
        for (i, example) in section.examples.iter().enumerate() {
            let label = match example.kind {
                Kind::Runs(_) => String::new(),
                _ => format!("  [{}]", example.kind.label()),
            };
            println!("  {:<6} {}{}", section.slide_id(i), example.title, label);
        }
    }
}

enum Selection {
    Everything,
    Section(usize),
    Slide(usize, usize),
    Titles(String),
}

impl Selection {
    fn parse(query: &str) -> Option<Selection> {
        let selection = if let Some((section, slide)) = query.split_once('.') {
            Selection::Slide(section.parse().ok()?, slide.parse().ok()?)
        } else if let Ok(number) = query.parse() {
            Selection::Section(number)
        } else {
            Selection::Titles(query.to_lowercase())
        };
        // Refuse selections that match nothing, so typos get an error message.
        let matches_something = SECTIONS
            .iter()
            .any(|s| (0..s.examples.len()).any(|i| selection.includes(s, i)));
        matches_something.then_some(selection)
    }

    fn includes(&self, section: &Section, index: usize) -> bool {
        match self {
            Selection::Everything => true,
            Selection::Section(number) => section.number == *number,
            Selection::Slide(number, slide) => section.number == *number && index + 1 == *slide,
            Selection::Titles(text) => section.title.to_lowercase().contains(text.as_str()),
        }
    }
}

const BOLD: &str = "1";
const DIM: &str = "2";
const RED: &str = "31";
const YELLOW: &str = "33";

/// Wrap `text` in an ANSI style when writing to a terminal (and NO_COLOR is unset).
fn paint(style: &str, text: &str) -> String {
    static COLOR: OnceLock<bool> = OnceLock::new();
    let color =
        *COLOR.get_or_init(|| std::io::stdout().is_terminal() && env::var_os("NO_COLOR").is_none());
    if color {
        format!("\x1b[{style}m{text}\x1b[0m")
    } else {
        text.to_string()
    }
}
