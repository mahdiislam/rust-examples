//! The code of every slide that compiles, one module per section and one function per slide.
//!
//! Inside each function, the slide's code appears exactly as it does on the slide.
//! Lines marked `---- setup ----` bring in what earlier slides defined, and lines
//! after `---- check ----` print and assert the results the slide's comments claim.
//! Neither is on the slide.
//!
//! The slides are small fragments meant for reading, so they leave values unused,
//! write `return` where it isn't needed, and so on. The lints below are silenced
//! for that reason, and rustfmt leaves these files alone so the code keeps the
//! exact layout it has on the slides.

#![allow(
    dead_code,
    unused_variables,
    unused_assignments,
    unused_mut,
    unused_must_use,
    unused_imports,
    path_statements,
    invalid_from_utf8,
    clippy::needless_return,
    clippy::redundant_field_names,
    clippy::no_effect,
    clippy::let_and_return,
    clippy::question_mark,
    clippy::needless_late_init,
    clippy::unused_unit,
    clippy::unit_arg,
    clippy::clone_on_copy,
    clippy::blocks_in_conditions,
    clippy::match_like_matches_macro,
    clippy::needless_borrow,
    clippy::approx_constant,
    clippy::bool_comparison,
    clippy::unnecessary_operation,
    clippy::extra_unused_type_parameters,
    clippy::needless_lifetimes,
    clippy::explicit_auto_deref,
    clippy::double_parens,
    clippy::print_literal,
    clippy::manual_range_contains,
    clippy::let_unit_value,
    clippy::almost_swapped,
    clippy::wrong_self_convention,
    clippy::non_canonical_clone_impl,
    clippy::vec_init_then_push,
    clippy::unnecessary_literal_unwrap,
    clippy::useless_vec
)]

#[rustfmt::skip] pub mod s01_variable_bindings;
#[rustfmt::skip] pub mod s02_tuples;
#[rustfmt::skip] pub mod s03_statements_and_functions;
#[rustfmt::skip] pub mod s04_blocks_and_expressions;
#[rustfmt::skip] pub mod s05_fields_methods_and_modules;
#[rustfmt::skip] pub mod s06_structs;
#[rustfmt::skip] pub mod s07_patterns;
#[rustfmt::skip] pub mod s08_methods_and_mutability;
#[rustfmt::skip] pub mod s09_traits;
#[rustfmt::skip] pub mod s10_copy_move_and_borrow;
#[rustfmt::skip] pub mod s11_generics;
#[rustfmt::skip] pub mod s12_macros_and_panics;
#[rustfmt::skip] pub mod s13_enums;
#[rustfmt::skip] pub mod s14_lifetimes;
#[rustfmt::skip] pub mod s15_static_and_ownership;
#[rustfmt::skip] pub mod s16_slices;
#[rustfmt::skip] pub mod s17_error_handling;
#[rustfmt::skip] pub mod s18_dereferencing;
#[rustfmt::skip] pub mod s19_closures;
#[rustfmt::skip] pub mod s20_loops_and_iterators;
#[rustfmt::skip] pub mod s21_returning_closures;
