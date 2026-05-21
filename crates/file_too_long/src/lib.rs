#![feature(rustc_private)]
#![warn(unused_extern_crates)]

extern crate rustc_ast;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

use rustc_ast::ast;
use rustc_lint::{EarlyContext, EarlyLintPass, LintContext};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{FileNameDisplayPreference, Span, Symbol};

const MAX_SOURCE_FILE_LINES: usize = 500;
const TOKEN_PALETTE_PATH: &str = "std-egui/src/tokens/palette.rs";

fn exceeds_source_file_limit(lines: usize) -> bool {
    lines > MAX_SOURCE_FILE_LINES
}

fn is_forbidden_color_constructor(path: &str) -> bool {
    matches!(
        path,
        "Color32::from_rgb"
            | "egui::Color32::from_rgb"
            | "Color32::from_rgba_premultiplied"
            | "egui::Color32::from_rgba_premultiplied"
            | "Color32::from_rgba_unmultiplied"
            | "egui::Color32::from_rgba_unmultiplied"
    )
}

fn is_palette_definition(path: &str) -> bool {
    path.ends_with(TOKEN_PALETTE_PATH)
}

fn path_to_string(path: &ast::Path) -> String {
    path.segments
        .iter()
        .map(|segment| segment.ident.name)
        .collect::<Vec<Symbol>>()
        .iter()
        .map(Symbol::as_str)
        .collect::<Vec<_>>()
        .join("::")
}

declare_lint! {
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

declare_lint! {
    /// ### What it does
    ///
    /// Reports inline egui Color32 RGB/RGBA constructors outside the token palette.
    ///
    /// ### Why is this bad?
    ///
    /// Launcher and Studio visual values must go through std-egui tokens so light,
    /// dark, high contrast and reduce transparency modes stay coherent.
    ///
    /// ### Known problems
    ///
    /// This lint intentionally starts with color constructors only. Size and spacing
    /// constructors remain covered by std doctor until they are migrated.
    ///
    /// ### Example
    ///
    /// ```rust
    /// let color = egui::Color32::from_rgb(28, 30, 34);
    /// ```
    ///
    /// Use instead:
    ///
    /// ```rust
    /// let color = std_egui::tokens::Color::bg_surface_0(ctx);
    /// ```
    pub NO_INLINE_VISUAL_VALUES,
    Warn,
    "inline visual colors must use std-egui tokens"
}

declare_lint_pass!(FileTooLong => [FILE_TOO_LONG, NO_INLINE_VISUAL_VALUES]);

dylint_linting::dylint_library!();

#[unsafe(no_mangle)]
pub fn register_lints(sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(sess);
    lint_store.register_lints(&[FILE_TOO_LONG, NO_INLINE_VISUAL_VALUES]);
    lint_store.register_early_pass(|| Box::new(FileTooLong));
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

    fn check_expr(&mut self, cx: &EarlyContext<'_>, expr: &ast::Expr) {
        let ast::ExprKind::Call(callee, _) = &expr.kind else {
            return;
        };
        let ast::ExprKind::Path(_, path) = &callee.kind else {
            return;
        };
        let source_path = cx
            .sess()
            .source_map()
            .span_to_filename(expr.span)
            .display(FileNameDisplayPreference::Local)
            .to_string();
        if is_palette_definition(&source_path) {
            return;
        }
        if !is_forbidden_color_constructor(&path_to_string(path)) {
            return;
        }
        cx.span_lint(NO_INLINE_VISUAL_VALUES, expr.span, |diag| {
            diag.primary_message("inline Color32 RGB/RGBA constructor must use token palette");
        });
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
fn inline_visual_color_lint_matches_egui_rgb_constructors() {
    assert!(is_forbidden_color_constructor("Color32::from_rgb"));
    assert!(is_forbidden_color_constructor("egui::Color32::from_rgb"));
    assert!(is_forbidden_color_constructor(
        "Color32::from_rgba_premultiplied"
    ));
    assert!(!is_forbidden_color_constructor("Color::bg_surface_0"));
}

#[test]
fn inline_visual_color_lint_allows_token_palette() {
    assert!(is_palette_definition("crates/std-egui/src/tokens/palette.rs"));
    assert!(!is_palette_definition("crates/std-launcher/src/ui.rs"));
}

#[test]
fn ui() {
    dylint_testing::ui_test(env!("CARGO_PKG_NAME"), "ui");
}
