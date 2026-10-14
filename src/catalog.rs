//! Every slide of the deck, in deck order.
//!
//! Titles and captions are copied from the slides. Each entry says whether the
//! slide's code runs, panics, must fail to compile, or is a slide of notes.

use crate::sections::*;
use crate::{Example, Kind, Section};

pub const SECTIONS: &[Section] = &[
    Section {
        number: 1,
        title: "Variable bindings",
        examples: &[
            Example {
                title: "let: declare, then assign",
                caption: "`let` introduces a variable binding.",
                kind: Kind::Runs(s01_variable_bindings::let_declare_then_assign),
            },
            Example {
                title: "let: on one line",
                caption: "Declaration and assignment usually happen together.",
                kind: Kind::Runs(s01_variable_bindings::let_on_one_line),
            },
            Example {
                title: "Type annotations",
                caption: "`:` annotates the type explicitly.",
                kind: Kind::Runs(s01_variable_bindings::type_annotations),
            },
            Example {
                title: "Annotated, on one line",
                caption: "Same thing, in a single statement.",
                kind: Kind::Runs(s01_variable_bindings::annotated_on_one_line),
            },
            Example {
                title: "Uninitialized: not allowed",
                caption: "The compiler refuses to let you read a binding before it is assigned.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/01_05_uninitialized_not_allowed.rs",
                    source: include_str!("../does_not_compile/01_05_uninitialized_not_allowed.rs"),
                    error: "E0381",
                },
            },
            Example {
                title: "Uninitialized: assign first",
                caption: "Assigning before use is fine, and the type can be inferred later.",
                kind: Kind::Runs(s01_variable_bindings::uninitialized_assign_first),
            },
            Example {
                title: "Throwing values away",
                caption: "`_` is not a name: it means \"discard this\".",
                kind: Kind::Runs(s01_variable_bindings::throwing_values_away),
            },
            Example {
                title: "Underscore-prefixed names",
                caption: "`_name` is a real binding; it just silences the unused-variable warning.",
                kind: Kind::Runs(s01_variable_bindings::underscore_prefixed_names),
            },
            Example {
                title: "Shadowing",
                caption: "A new binding can reuse an existing name.",
                kind: Kind::Runs(s01_variable_bindings::shadowing),
            },
        ],
    },
    Section {
        number: 2,
        title: "Tuples",
        examples: &[
            Example {
                title: "Tuples",
                caption: "Fixed-length collections of values that may have different types.",
                kind: Kind::Runs(s02_tuples::tuples),
            },
            Example {
                title: "Annotating a tuple",
                caption: "Tuple types are written like tuple values.",
                kind: Kind::Runs(s02_tuples::annotating_a_tuple),
            },
            Example {
                title: "Destructuring a tuple",
                caption: "A `let` pattern breaks a tuple into its fields.",
                kind: Kind::Runs(s02_tuples::destructuring_a_tuple),
            },
            Example {
                title: "Destructuring a returned tuple",
                caption: "Handy when a function returns several values.",
                kind: Kind::Runs(s02_tuples::destructuring_a_returned_tuple),
            },
            Example {
                title: "Ignoring part of a tuple",
                caption: "`_` discards the fields you don't need.",
                kind: Kind::Runs(s02_tuples::ignoring_part_of_a_tuple),
            },
        ],
    },
    Section {
        number: 3,
        title: "Statements & functions",
        examples: &[
            Example {
                title: "Statements",
                caption: "A semicolon ends a statement.",
                kind: Kind::Runs(s03_statements_and_functions::statements),
            },
            Example {
                title: "Multi-line statements",
                caption: "Since `;` ends the statement, one statement can span many lines.",
                kind: Kind::Runs(s03_statements_and_functions::multi_line_statements),
            },
            Example {
                title: "A function with no return value",
                caption: "`fn` declares a function.",
                kind: Kind::Runs(s03_statements_and_functions::a_function_with_no_return_value),
            },
            Example {
                title: "A function that returns a value",
                caption: "`->` introduces the return type.",
                kind: Kind::Runs(s03_statements_and_functions::a_function_that_returns_a_value),
            },
        ],
    },
    Section {
        number: 4,
        title: "Blocks & expressions",
        examples: &[
            Example {
                title: "Blocks have their own scope",
                caption: "Braces open a new scope.",
                kind: Kind::Runs(s04_blocks_and_expressions::blocks_have_their_own_scope),
            },
            Example {
                title: "Blocks are expressions",
                caption: "A block evaluates to a value.",
                kind: Kind::Runs(s04_blocks_and_expressions::blocks_are_expressions),
            },
            Example {
                title: "The tail of a block",
                caption: "Statements run in order; the last expression is the block's value.",
                kind: Kind::Runs(s04_blocks_and_expressions::the_tail_of_a_block),
            },
            Example {
                title: "Implicit return",
                caption: "Leaving off the final semicolon returns the value.",
                kind: Kind::Runs(s04_blocks_and_expressions::implicit_return),
            },
            Example {
                title: "if is an expression",
                caption: "Each branch produces a value.",
                kind: Kind::Runs(s04_blocks_and_expressions::if_is_an_expression),
            },
            Example {
                title: "match is an expression",
                caption: "So is `match`.",
                kind: Kind::Runs(s04_blocks_and_expressions::match_is_an_expression),
            },
        ],
    },
    Section {
        number: 5,
        title: "Fields, methods & modules",
        examples: &[
            Example {
                title: "Field access",
                caption: "A dot reaches into a value's fields.",
                kind: Kind::Runs(s05_fields_methods_and_modules::field_access),
            },
            Example {
                title: "Method calls",
                caption: "A dot also calls methods.",
                kind: Kind::Runs(s05_fields_methods_and_modules::method_calls),
            },
            Example {
                title: "Paths with ::",
                caption: "`::` navigates namespaces: crate → module → item.",
                kind: Kind::Runs(s05_fields_methods_and_modules::paths_with_double_colon),
            },
            Example {
                title: "use brings names into scope",
                caption: "",
                kind: Kind::Runs(s05_fields_methods_and_modules::use_brings_names_into_scope),
            },
            Example {
                title: "Importing several names",
                caption: "Inside `use`, braces group several imports.",
                kind: Kind::Runs(s05_fields_methods_and_modules::importing_several_names),
            },
            Example {
                title: "Glob imports",
                caption: "`*` imports everything public from a module.",
                kind: Kind::Runs(s05_fields_methods_and_modules::glob_imports),
            },
            Example {
                title: "Types are namespaces too",
                caption: "Methods can be called like plain functions.",
                kind: Kind::Runs(s05_fields_methods_and_modules::types_are_namespaces_too),
            },
            Example {
                title: "The prelude",
                caption: "Common non-primitive types are in scope by default.",
                kind: Kind::Runs(s05_fields_methods_and_modules::the_prelude),
            },
            Example {
                title: "What the prelude does",
                caption: "Every module implicitly starts with a prelude import (per edition).",
                kind: Kind::Runs(s05_fields_methods_and_modules::what_the_prelude_does),
            },
        ],
    },
    Section {
        number: 6,
        title: "Structs",
        examples: &[
            Example {
                title: "Declaring a struct",
                caption: "`struct` defines a type with named fields.",
                kind: Kind::Runs(s06_structs::declaring_a_struct),
            },
            Example {
                title: "Struct literals",
                caption: "Fields can be listed in any order; names are what matter.",
                kind: Kind::Runs(s06_structs::struct_literals),
            },
            Example {
                title: "Struct update syntax",
                caption: "`..other` fills in the remaining fields. It must come last, with no trailing comma.",
                kind: Kind::Runs(s06_structs::struct_update_syntax),
            },
            Example {
                title: "Update syntax for every field",
                caption: "\"The remaining fields\" can be all of them.",
                kind: Kind::Runs(s06_structs::update_syntax_for_every_field),
            },
            Example {
                title: "Destructuring a struct",
                caption: "Structs work in `let` patterns, just like tuples.",
                kind: Kind::Runs(s06_structs::destructuring_a_struct),
            },
            Example {
                title: "Ignoring the rest",
                caption: "`..` skips fields you don't name.",
                kind: Kind::Runs(s06_structs::ignoring_the_rest),
            },
        ],
    },
    Section {
        number: 7,
        title: "Patterns",
        examples: &[
            Example {
                title: "if let",
                caption: "A `let` pattern can be used as a condition.",
                kind: Kind::Runs(s07_patterns::if_let),
            },
            Example {
                title: "match arms are patterns",
                caption: "Same logic, written with `match`.",
                kind: Kind::Runs(s07_patterns::match_arms_are_patterns),
            },
            Example {
                title: "Matches must be exhaustive",
                caption: "Every possible value has to be covered by some arm.",
                kind: Kind::Runs(s07_patterns::matches_must_be_exhaustive),
            },
            Example {
                title: "The catch-all pattern",
                caption: "`_` matches anything.",
                kind: Kind::Runs(s07_patterns::the_catch_all_pattern),
            },
        ],
    },
    Section {
        number: 8,
        title: "Methods & mutability",
        examples: &[
            Example {
                title: "Declaring methods",
                caption: "`impl` attaches methods to your own types.",
                kind: Kind::Runs(s08_methods_and_mutability::declaring_methods),
            },
            Example {
                title: "Calling methods",
                caption: "",
                kind: Kind::Runs(s08_methods_and_mutability::calling_methods),
            },
            Example {
                title: "Immutable by default: fields",
                caption: "Fields of an immutable binding can't be changed.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/08_03_immutable_by_default_fields.rs",
                    source: include_str!(
                        "../does_not_compile/08_03_immutable_by_default_fields.rs"
                    ),
                    error: "E0594",
                },
            },
            Example {
                title: "Immutable by default: reassignment",
                caption: "Nor can the binding be reassigned.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/08_04_immutable_by_default_reassignment.rs",
                    source: include_str!(
                        "../does_not_compile/08_04_immutable_by_default_reassignment.rs"
                    ),
                    error: "E0384",
                },
            },
            Example {
                title: "mut",
                caption: "`mut` makes a binding mutable.",
                kind: Kind::Runs(s08_methods_and_mutability::mut_),
            },
        ],
    },
    Section {
        number: 9,
        title: "Traits",
        examples: &[
            Example {
                title: "Declaring a trait",
                caption: "Traits describe behaviour that several types can share.",
                kind: Kind::Runs(s09_traits::declaring_a_trait),
            },
            Example {
                title: "Orphan rules",
                caption: "You may implement:",
                kind: Kind::Notes(&[
                    "your trait on anyone's type",
                    "anyone's trait on your type",
                    "but not a foreign trait on a foreign type",
                ]),
            },
            Example {
                title: "Our trait, our type",
                caption: "",
                kind: Kind::Runs(s09_traits::our_trait_our_type),
            },
            Example {
                title: "Our trait, a foreign type",
                caption: "Even a primitive type.",
                kind: Kind::Runs(s09_traits::our_trait_a_foreign_type),
            },
            Example {
                title: "A foreign trait, our type",
                caption: "`Neg` overloads the unary minus operator.",
                kind: Kind::Runs(s09_traits::a_foreign_trait_our_type),
            },
            Example {
                title: "Self",
                caption: "Inside an `impl` block, `Self` is the type being implemented.",
                kind: Kind::Runs(s09_traits::self_),
            },
        ],
    },
    Section {
        number: 10,
        title: "Copy, move & borrow",
        examples: &[
            Example {
                title: "Copy types are copied",
                caption: "Marker traits say what can be done with a type. `u64` is `Copy`:",
                kind: Kind::Runs(s10_copy_move_and_borrow::copy_types_are_copied),
            },
            Example {
                title: "Copy into functions",
                caption: "Passing a `Copy` value also copies it.",
                kind: Kind::Runs(s10_copy_move_and_borrow::copy_into_functions),
            },
            Example {
                title: "Non-Copy types are moved",
                caption: "`Temp` is not `Copy`, so assignment moves it.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/10_03_non_copy_types_are_moved.rs",
                    source: include_str!("../does_not_compile/10_03_non_copy_types_are_moved.rs"),
                    error: "E0382",
                },
            },
            Example {
                title: "Moving into a function",
                caption: "Passing by value moves, too.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/10_04_moving_into_a_function.rs",
                    source: include_str!("../does_not_compile/10_04_moving_into_a_function.rs"),
                    error: "E0382",
                },
            },
            Example {
                title: "Borrowing with &",
                caption: "An immutable reference lends the value instead of moving it.",
                kind: Kind::Runs(s10_copy_move_and_borrow::borrowing_with_ref),
            },
            Example {
                title: "Borrowing with &mut",
                caption: "A mutable reference requires a `mut` binding, and is always explicit.",
                kind: Kind::Runs(s10_copy_move_and_borrow::borrowing_with_mut),
            },
            Example {
                title: "Receivers by reference",
                caption: "Trait methods can take `&self` or `&mut self`.",
                kind: Kind::Runs(s10_copy_move_and_borrow::receivers_by_reference),
            },
            Example {
                title: "Implicit borrowing of the receiver",
                caption: "Method syntax borrows the receiver for you.",
                kind: Kind::Runs(s10_copy_move_and_borrow::implicit_borrowing_of_the_receiver),
            },
            Example {
                title: "Method call = function call",
                caption: "These two lines are equivalent.",
                kind: Kind::Runs(s10_copy_move_and_borrow::method_call_function_call),
            },
            Example {
                title: "Implementing Copy",
                caption: "Marker traits like `Copy` have no methods.",
                kind: Kind::Runs(s10_copy_move_and_borrow::implementing_copy),
            },
            Example {
                title: "Clone still works",
                caption: "",
                kind: Kind::Runs(s10_copy_move_and_borrow::clone_still_works),
            },
            Example {
                title: "...and nothing is moved anymore",
                caption: "",
                kind: Kind::Runs(s10_copy_move_and_borrow::and_nothing_is_moved_anymore),
            },
            Example {
                title: "Deriving traits",
                caption: "`derive` generates common trait implementations for you.",
                kind: Kind::Runs(s10_copy_move_and_borrow::deriving_traits),
            },
        ],
    },
    Section {
        number: 11,
        title: "Generics",
        examples: &[
            Example {
                title: "Generic functions",
                caption: "Type parameters go in angle brackets.",
                kind: Kind::Runs(s11_generics::generic_functions),
            },
            Example {
                title: "Several type parameters",
                caption: "",
                kind: Kind::Runs(s11_generics::several_type_parameters),
            },
            Example {
                title: "Trait bounds",
                caption: "Constraints say what a type parameter can do.",
                kind: Kind::Runs(s11_generics::trait_bounds),
            },
            Example {
                title: "where clauses",
                caption: "A longer syntax for the same constraints.",
                kind: Kind::Runs(s11_generics::where_clauses),
            },
            Example {
                title: "Multiple bounds",
                caption: "`+` requires several traits at once.",
                kind: Kind::Runs(s11_generics::multiple_bounds),
            },
            Example {
                title: "Turbofish",
                caption: "Generic functions can be navigated with `::<>`, the turbofish.",
                kind: Kind::Runs(s11_generics::turbofish),
            },
            Example {
                title: "Generic structs",
                caption: "Structs can take type parameters too.",
                kind: Kind::Runs(s11_generics::generic_structs),
            },
            Example {
                title: "Vec<T> is generic",
                caption: "The standard heap-allocated array.",
                kind: Kind::Runs(s11_generics::vec_t_is_generic),
            },
            Example {
                title: "vec! literals",
                caption: "The `vec!` macro builds a `Vec` from a list.",
                kind: Kind::Runs(s11_generics::vec_literals),
            },
        ],
    },
    Section {
        number: 12,
        title: "Macros & panics",
        examples: &[
            Example {
                title: "println! is a macro",
                caption: "`name!()`, `name![]` and `name!{}` all invoke macros.",
                kind: Kind::Runs(s12_macros_and_panics::println_is_a_macro),
            },
            Example {
                title: "Macros expand to regular code",
                caption: "Roughly what that `println!` does:",
                kind: Kind::Runs(s12_macros_and_panics::macros_expand_to_regular_code),
            },
            Example {
                title: "panic!",
                caption: "Stops the thread with a message and its location.",
                kind: Kind::Panics(s12_macros_and_panics::panic),
            },
            Example {
                title: "Functions that panic",
                caption: "`.unwrap()` on an empty `Option` panics.",
                kind: Kind::Panics(s12_macros_and_panics::functions_that_panic),
            },
        ],
    },
    Section {
        number: 13,
        title: "Enums",
        examples: &[
            Example {
                title: "Option is an enum",
                caption: "An enum is a type with several variants.",
                kind: Kind::Runs(s13_enums::option_is_an_enum),
            },
            Example {
                title: "Result is an enum too",
                caption: "A success value or an error. Unwrapping an `Err` panics.",
                kind: Kind::Runs(s13_enums::result_is_an_enum_too),
            },
        ],
    },
    Section {
        number: 14,
        title: "Lifetimes",
        examples: &[
            Example {
                title: "Bindings have lifetimes",
                caption: "",
                kind: Kind::Runs(s14_lifetimes::bindings_have_lifetimes),
            },
            Example {
                title: "References have lifetimes",
                caption: "",
                kind: Kind::Runs(s14_lifetimes::references_have_lifetimes),
            },
            Example {
                title: "A reference can't outlive its value",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/14_03_a_reference_cant_outlive_its_value.rs",
                    source: include_str!(
                        "../does_not_compile/14_03_a_reference_cant_outlive_its_value.rs"
                    ),
                    error: "E0597",
                },
            },
            Example {
                title: "Many shared borrows",
                caption: "Any number of immutable borrows at once is fine.",
                kind: Kind::Runs(s14_lifetimes::many_shared_borrows),
            },
            Example {
                title: "No mutation while borrowed",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/14_05_no_mutation_while_borrowed.rs",
                    source: include_str!("../does_not_compile/14_05_no_mutation_while_borrowed.rs"),
                    error: "E0506",
                },
            },
            Example {
                title: "Shared XOR mutable",
                caption: "A value can't be mutably borrowed while immutably borrowed.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/14_06_shared_xor_mutable.rs",
                    source: include_str!("../does_not_compile/14_06_shared_xor_mutable.rs"),
                    error: "E0502",
                },
            },
            Example {
                title: "Lifetimes in function arguments",
                caption: "Reference arguments are borrowed for the whole call.",
                kind: Kind::Runs(s14_lifetimes::lifetimes_in_function_arguments),
            },
            Example {
                title: "Named lifetimes",
                caption: "Functions taking references are generic over lifetimes, written `'a`.",
                kind: Kind::Runs(s14_lifetimes::named_lifetimes),
            },
            Example {
                title: "Returning a borrowed reference",
                caption: "The output lifetime is tied to the input's.",
                kind: Kind::Runs(s14_lifetimes::returning_a_borrowed_reference),
            },
            Example {
                title: "Lifetime elision",
                caption: "With a single input lifetime, it doesn't need a name.",
                kind: Kind::Runs(s14_lifetimes::lifetime_elision),
            },
            Example {
                title: "Structs generic over lifetimes",
                caption: "A struct can hold references if it declares a lifetime.",
                kind: Kind::Runs(s14_lifetimes::structs_generic_over_lifetimes),
            },
            Example {
                title: "...built by a function",
                caption: "",
                kind: Kind::Runs(s14_lifetimes::built_by_a_function),
            },
            Example {
                title: "...with an elided lifetime",
                caption: "`'_` means \"the obvious lifetime\".",
                kind: Kind::Runs(s14_lifetimes::with_an_elided_lifetime),
            },
            Example {
                title: "impl blocks generic over lifetimes",
                caption: "",
                kind: Kind::Runs(s14_lifetimes::impl_blocks_generic_over_lifetimes),
            },
            Example {
                title: "Eliding in methods",
                caption: "",
                kind: Kind::Runs(s14_lifetimes::eliding_in_methods),
            },
            Example {
                title: "Eliding even harder",
                caption: "If you never use the name, skip it.",
                kind: Kind::Runs(s14_lifetimes::eliding_even_harder),
            },
        ],
    },
    Section {
        number: 15,
        title: "'static & ownership",
        examples: &[
            Example {
                title: "The 'static lifetime",
                caption: "Valid for the whole program. String literals are `'static`.",
                kind: Kind::Runs(s15_static_and_ownership::the_static_lifetime),
            },
            Example {
                title: "A String is not 'static",
                caption: "It is heap-allocated and freed when its owner goes away.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/15_02_a_string_is_not_static.rs",
                    source: include_str!("../does_not_compile/15_02_a_string_is_not_static.rs"),
                    error: "E0597",
                },
            },
            Example {
                title: "Fix A: be generic over a lifetime",
                caption: "",
                kind: Kind::Runs(s15_static_and_ownership::fix_a_be_generic_over_a_lifetime),
            },
            Example {
                title: "Fix B: take ownership",
                caption: "",
                kind: Kind::Runs(s15_static_and_ownership::fix_b_take_ownership),
            },
            Example {
                title: "Field init shorthand",
                caption: "When a field and a variable share a name, write it once. Clippy will suggest this.",
                kind: Kind::Runs(s15_static_and_ownership::field_init_shorthand),
            },
            Example {
                title: "Owned vs. reference types",
                caption: "Many types come in two flavours:",
                kind: Kind::Notes(&[
                    "`String` is owned, `&str` is a reference",
                    "`PathBuf` is owned, `&Path` is a reference",
                    "`Vec<T>` is owned, `&[T]` is a reference",
                ]),
            },
        ],
    },
    Section {
        number: 16,
        title: "Slices",
        examples: &[
            Example {
                title: "Slices",
                caption: "A reference to a run of contiguous elements.",
                kind: Kind::Runs(s16_slices::slices),
            },
            Example {
                title: "Ranges",
                caption: "Indexing is just the `Index` trait; `..` builds range values.",
                kind: Kind::Runs(s16_slices::ranges),
            },
            Example {
                title: "Borrowing rules apply to slices",
                caption: "",
                kind: Kind::Runs(s16_slices::borrowing_rules_apply_to_slices),
            },
            Example {
                title: "...which really means",
                caption: "",
                kind: Kind::Runs(s16_slices::which_really_means),
            },
            Example {
                title: "Legal: a 'static array",
                caption: "This compiles only because the literal array is `'static`.",
                kind: Kind::Runs(s16_slices::legal_a_static_array),
            },
            Example {
                title: "Illegal: a heap-allocated Vec",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/16_06_illegal_a_heap_allocated_vec.rs",
                    source: include_str!(
                        "../does_not_compile/16_06_illegal_a_heap_allocated_vec.rs"
                    ),
                    error: "E0597",
                },
            },
            Example {
                title: "String slices",
                caption: "`&str` values are slices too.",
                kind: Kind::Runs(s16_slices::string_slices),
            },
            Example {
                title: "...so borrow rules apply",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/16_08_so_borrow_rules_apply.rs",
                    source: include_str!("../does_not_compile/16_08_so_borrow_rules_apply.rs"),
                    error: "E0597",
                },
            },
        ],
    },
    Section {
        number: 17,
        title: "Error handling",
        examples: &[
            Example {
                title: "Fallible functions return Result",
                caption: "",
                kind: Kind::Runs(s17_error_handling::fallible_functions_return_result),
            },
            Example {
                title: ".unwrap()",
                caption: "Panic on failure.",
                kind: Kind::Panics(s17_error_handling::unwrap),
            },
            Example {
                title: ".expect()",
                caption: "Panic with your own message.",
                kind: Kind::Panics(s17_error_handling::expect),
            },
            Example {
                title: "match on a Result",
                caption: "",
                kind: Kind::Runs(s17_error_handling::match_on_a_result),
            },
            Example {
                title: "if let on a Result",
                caption: "",
                kind: Kind::Runs(s17_error_handling::if_let_on_a_result),
            },
            Example {
                title: "Bubbling the error up",
                caption: "Return the error to the caller.",
                kind: Kind::Runs(s17_error_handling::bubbling_the_error_up),
            },
            Example {
                title: "The ? operator",
                caption: "The concise way to bubble errors up.",
                kind: Kind::Runs(s17_error_handling::the_question_mark_operator),
            },
        ],
    },
    Section {
        number: 18,
        title: "Dereferencing",
        examples: &[
            Example {
                title: "Auto-deref",
                caption: "No `*` needed to read fields or call methods through a reference.",
                kind: Kind::Runs(s18_dereferencing::auto_deref),
            },
            Example {
                title: "Explicit * needs Copy",
                caption: "Dereferencing would move out of the reference.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/18_02_explicit_deref_needs_copy.rs",
                    source: include_str!("../does_not_compile/18_02_explicit_deref_needs_copy.rs"),
                    error: "E0507",
                },
            },
            Example {
                title: "...with Copy, it works",
                caption: "",
                kind: Kind::Runs(s18_dereferencing::with_copy_it_works),
            },
        ],
    },
    Section {
        number: 19,
        title: "Closures",
        examples: &[
            Example {
                title: "Closures",
                caption: "Functions of type `Fn`, `FnMut` or `FnOnce`, with captured context.",
                kind: Kind::Runs(s19_closures::closures),
            },
            Example {
                title: "Closures borrow",
                caption: "A closure that captures a local borrows it.",
                kind: Kind::Runs(s19_closures::closures_borrow),
            },
            Example {
                title: "...which can conflict with 'static",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/19_03_which_can_conflict_with_static.rs",
                    source: include_str!(
                        "../does_not_compile/19_03_which_can_conflict_with_static.rs"
                    ),
                    error: "E0373",
                },
            },
            Example {
                title: "move closures",
                caption: "`move` transfers ownership of captures into the closure.",
                kind: Kind::Runs(s19_closures::move_closures),
            },
            Example {
                title: "Fn can be called freely",
                caption: "",
                kind: Kind::Runs(s19_closures::fn_can_be_called_freely),
            },
            Example {
                title: "FnMut: one call at a time",
                caption: "Calling an `FnMut` borrows it mutably.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/19_06_fnmut_one_call_at_a_time.rs",
                    source: include_str!("../does_not_compile/19_06_fnmut_one_call_at_a_time.rs"),
                    error: "E0499",
                },
            },
            Example {
                title: "FnMut: sequential calls are fine",
                caption: "",
                kind: Kind::Runs(s19_closures::fnmut_sequential_calls_are_fine),
            },
            Example {
                title: "Why FnMut exists",
                caption: "Some closures mutate what they capture.",
                kind: Kind::Runs(s19_closures::why_fnmut_exists),
            },
            Example {
                title: "Fn rejects mutating closures",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/19_09_fn_rejects_mutating_closures.rs",
                    source: include_str!(
                        "../does_not_compile/19_09_fn_rejects_mutating_closures.rs"
                    ),
                    error: "E0594",
                },
            },
            Example {
                title: "FnOnce",
                caption: "Closures that give away what they captured can only run once.",
                kind: Kind::Runs(s19_closures::fnonce),
            },
            Example {
                title: "FnOnce can't be called twice",
                caption: "Calling it consumes it.",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/19_11_fnonce_cant_be_called_twice.rs",
                    source: include_str!(
                        "../does_not_compile/19_11_fnonce_cant_be_called_twice.rs"
                    ),
                    error: "E0382",
                },
            },
            Example {
                title: "...and it really moves",
                caption: "",
                kind: Kind::DoesNotCompile {
                    file: "does_not_compile/19_12_and_it_really_moves.rs",
                    source: include_str!("../does_not_compile/19_12_and_it_really_moves.rs"),
                    error: "E0382",
                },
            },
            Example {
                title: "Cloning instead",
                caption: "",
                kind: Kind::Runs(s19_closures::cloning_instead),
            },
            Example {
                title: "Closures with two arguments",
                caption: "",
                kind: Kind::Runs(s19_closures::closures_with_two_arguments),
            },
            Example {
                title: "Ignoring arguments",
                caption: "",
                kind: Kind::Panics(s19_closures::ignoring_arguments),
            },
            Example {
                title: "A closure in a loop",
                caption: "",
                kind: Kind::Runs(s19_closures::a_closure_in_a_loop),
            },
            Example {
                title: "The toilet closure",
                caption: "Takes anything, returns nothing. `|_| ()` looks like a toilet.",
                kind: Kind::Runs(s19_closures::the_toilet_closure),
            },
        ],
    },
    Section {
        number: 20,
        title: "Loops & iterators",
        examples: &[
            Example {
                title: "for over a Vec",
                caption: "Anything iterable works with `for ... in`.",
                kind: Kind::Runs(s20_loops_and_iterators::for_over_a_vec),
            },
            Example {
                title: "for over a slice",
                caption: "",
                kind: Kind::Runs(s20_loops_and_iterators::for_over_a_slice),
            },
            Example {
                title: "for over an iterator",
                caption: "`char` is a Unicode scalar value; `&str` also has `.bytes()`.",
                kind: Kind::Runs(s20_loops_and_iterators::for_over_an_iterator),
            },
            Example {
                title: "Iterator adapters",
                caption: "Filtered, mapped and flattened iterators work too.",
                kind: Kind::Runs(s20_loops_and_iterators::iterator_adapters),
            },
        ],
    },
    Section {
        number: 21,
        title: "Returning closures",
        examples: &[
            Example {
                title: "Returning a closure",
                caption: "`impl Fn(...)` names \"some closure type\".",
                kind: Kind::Runs(s21_returning_closures::returning_a_closure),
            },
            Example {
                title: "Capturing a borrowed argument",
                caption: "The returned closure can hold a reference, with a lifetime bound.",
                kind: Kind::Runs(s21_returning_closures::capturing_a_borrowed_argument),
            },
            Example {
                title: "...with elided lifetimes",
                caption: "",
                kind: Kind::Runs(s21_returning_closures::with_elided_lifetimes),
            },
        ],
    },
];
