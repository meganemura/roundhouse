//! `form_with url:, scope:` over the native HTTP server; the Ruby-lane
//! twin is `emit_and_run/form_with_scope.rs`.

use super::form_with_scope_contract as contract;
use super::native_http;

#[test]
#[ignore = "requires Spinel; run in its CI lane"]
fn a_scope_form_renders_from_params_without_a_record_natively() {
    let tree = contract::overlay().native_http_tree();
    let server = native_http::Server::start(&tree);
    let mut diffs = Vec::new();
    for case in contract::CASES {
        let headers: &[(&str, &str)] =
            if case.method == "POST" { &[("Content-Type", "application/x-www-form-urlencoded")] } else { &[] };
        let response = server.request(case.method, case.path, headers, case.form);
        diffs.extend(contract::differences(case, response.status, &response.body));
    }
    assert!(diffs.is_empty(), "{}\n{}", diffs.join("\n"), server.log());
}
