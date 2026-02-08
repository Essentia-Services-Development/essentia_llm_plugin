//! Essentia LLM Plugin library.


// TODO(FEATURE): Add comprehensive documentation to all public items
// Tracked in documentation remediation queue
#![allow(missing_docs)]
// LLM plugin pedantic lint allowances (LLM-LINT-STAGING-01)
// HTTP/API integration with many casts and string operations
#![allow(
    clippy::assigning_clones,
    clippy::bool_to_int_with_if,
    clippy::cast_lossless,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::elidable_lifetime_names,
    clippy::explicit_iter_loop,
    clippy::float_cmp,
    clippy::format_push_string,
    clippy::if_not_else,
    clippy::ignored_unit_patterns,
    clippy::implicit_clone,
    clippy::items_after_statements,
    clippy::let_underscore_untyped,
    clippy::manual_let_else,
    clippy::manual_string_new,
    clippy::many_single_char_names,
    clippy::map_unwrap_or,
    clippy::match_bool,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::missing_errors_doc,
    clippy::missing_fields_in_debug,
    clippy::missing_panics_doc,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::needless_pass_by_value,
    clippy::no_effect_underscore_binding,
    clippy::range_plus_one,
    clippy::redundant_closure_for_method_calls,
    clippy::redundant_else,
    clippy::return_self_not_must_use,
    clippy::self_only_used_in_recursion,
    clippy::semicolon_if_nothing_returned,
    clippy::similar_names,
    clippy::single_char_pattern,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::uninlined_format_args,
    clippy::unnecessary_literal_bound,
    clippy::unnecessary_wraps,
    clippy::unnested_or_patterns,
    clippy::unreadable_literal,
    clippy::unused_self,
    clippy::used_underscore_binding,
    clippy::wildcard_imports
)]

pub mod core;
pub mod essentia;
pub mod flexforge;

// Re-exports for convenience
pub use flexforge::{LlmPluginConfig, LlmPluginFlexForge, LlmProvider};