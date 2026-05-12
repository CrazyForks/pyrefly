/*
 * Copyright (c) Meta Platforms, Inc. and affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use itertools::Itertools;
use lsp_types::DocumentHighlightKind;
use pretty_assertions::assert_eq;
use pyrefly_build::handle::Handle;
use ruff_text_size::TextSize;

use crate::state::state::State;
use crate::test::util::code_frame_of_source_at_range;
use crate::test::util::get_batched_lsp_operations_report;
use crate::test::util::get_batched_lsp_operations_report_allow_error;

fn get_test_report(state: &State, handle: &Handle, position: TextSize) -> String {
    let transaction = state.transaction();
    let module_info = transaction.get_module_info(handle).unwrap();
    let highlights = transaction
        .find_local_references(handle, position, true)
        .into_iter()
        .map(|range| {
            let kind = if transaction
                .identifier_at(handle, range.start())
                .expect("local references should point at identifiers")
                .context
                .is_write()
            {
                DocumentHighlightKind::WRITE
            } else {
                DocumentHighlightKind::READ
            };
            format!(
                "{}:\n{}",
                if kind == DocumentHighlightKind::WRITE {
                    "DocumentHighlightKind::WRITE"
                } else {
                    "DocumentHighlightKind::READ"
                },
                code_frame_of_source_at_range(module_info.contents(), range)
            )
        })
        .join("\n");
    format!("Highlights:\n{highlights}")
}

#[test]
#[should_panic(expected = "local references should point at identifiers")]
fn document_highlight_crash_on_match_without_case() {
    // A match statement without a `case` keyword causes parser recovery to produce
    // an AST where `identifier_at` returns `None` for a reference range, crashing
    // the LSP's document_highlight handler.
    let code = r#"
class Reducer:
    def __init__(self, type: int) -> None:
        self.type = type

    def fit(self) -> None:
        match self.type:
            Reducer
#            ^
"#;
    let _ = get_batched_lsp_operations_report_allow_error(&[("main", code)], get_test_report);
}

#[test]
fn document_highlight_includes_read_write_kind() {
    let code = r#"
x = 1
y = x
#   ^
"#;
    let report = get_batched_lsp_operations_report(&[("main", code)], get_test_report);
    assert_eq!(
        r#"
# main.py
3 | y = x
        ^
Highlights:
DocumentHighlightKind::WRITE:
2 | x = 1
    ^
DocumentHighlightKind::READ:
3 | y = x
        ^
"#
        .trim(),
        report.trim(),
    );
}
