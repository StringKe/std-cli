#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_span;

use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_span::{FileNameDisplayPreference, Span};

const MAX_SOURCE_FILE_LINES: usize = 500;

fn exceeds_source_file_limit(lines: usize) -> bool {
    lines > MAX_SOURCE_FILE_LINES
}

dylint_linting::declare_early_lint! {
    /// ### What it does
    ///
    /// Reports Rust source files with more than 500 lines.
    ///
    /// ### Why is this bad?
    ///
    /// Very large source files hide module boundaries and slow down review.
    ///
    /// ### Known problems
    ///
    /// Generated sources should not live under workspace `src` trees.
    ///
    /// ### Example
    ///
    /// A Rust source file with more than 500 lines:
    /// 
    /// ```rust
    /// fn first() {}
    /// // ...
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// mod focused_domain;
    /// ```
    pub FILE_TOO_LONG,
    Warn,
    "Rust source file exceeds the maximum line count"
}

impl EarlyLintPass for FileTooLong {
    fn check_crate(&mut self, cx: &EarlyContext<'_>, _: &rustc_ast::Crate) {
        for file in cx.sess().source_map().files().iter() {
            if !file.is_real_file() || file.is_imported() {
                continue;
            }
            let path = file.name.display(FileNameDisplayPreference::Local).to_string();
            if !path.ends_with(".rs") {
                continue;
            }
            let lines = file.count_lines();
            if !exceeds_source_file_limit(lines) {
                continue;
            }
            cx.span_lint(FILE_TOO_LONG, Span::default(), |diag| {
                diag.primary_message(format!(
                    "{path} has {lines} lines, maximum is {MAX_SOURCE_FILE_LINES}"
                ));
            });
        }
    }
}

#[test]
fn line_limit_allows_500_lines() {
    assert!(!exceeds_source_file_limit(500));
}

#[test]
fn line_limit_rejects_501_lines() {
    assert!(exceeds_source_file_limit(501));
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
