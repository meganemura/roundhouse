//! `form_with url:, scope:` on the Ruby lane; the native twin is
//! `spinel_toolchain/form_with_scope.rs`.

use super::form_with_scope_contract as contract;

/// A scope form for a model with validations renders, before and after
/// a failed create, filled from `params` and with no error wrapping.
#[test]
fn a_scope_form_renders_from_params_without_a_record() {
    let run = contract::overlay().run_ruby(&contract::script());
    run.assert_passes();
    let responses = contract::responses(&run.stdout);
    assert_eq!(responses.len(), contract::CASES.len(), "{}", run.stdout);
    let diffs: Vec<String> = contract::CASES
        .iter()
        .zip(&responses)
        .flat_map(|(case, (status, body))| contract::differences(case, *status, body))
        .collect();
    assert!(diffs.is_empty(), "{}", diffs.join("\n"));
}
