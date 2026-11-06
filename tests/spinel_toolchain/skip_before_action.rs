//! `skip_before_action` over the native HTTP server; the Ruby-lane twin
//! is `emit_and_run/skip_before_action.rs`.

use super::native_http;
use super::skip_before_action_contract as contract;

#[test]
#[ignore = "requires Spinel; run in its CI lane"]
fn a_skip_narrows_a_before_action_natively() {
    let tree = contract::overlay().native_http_tree();
    let server = native_http::Server::start(&tree);
    for (path, body) in contract::CASES {
        let response = server.get(path);
        assert_eq!(response.status, 200, "{path}: {}\n{}", response.body, server.log());
        assert_eq!(response.body, *body, "{path}");
    }
}
