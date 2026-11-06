//! `skip_before_action` on the Ruby lane; the native twin is
//! `spinel_toolchain/skip_before_action.rs`.

use super::skip_before_action_contract as contract;

/// A before_action runs once, and only where Rails' chain still runs it
/// after every later `skip_before_action`: spliced into the controller's
/// own action, or called from a subclass's dispatcher.
#[test]
fn a_skip_narrows_a_before_action() {
    let run = contract::overlay().run_ruby(&contract::assertions());
    run.assert_passes();
    assert!(run.stdout.contains("skip_before_action contract passed"), "{}", run.stdout);
}
