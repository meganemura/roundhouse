//! The native half of `emit_and_run::method_assignment_operand`.

use super::method_assignment_operand_contract as contract;

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_method_assignment_operand_of_an_infix_operator_keeps_its_parentheses_natively() {
    let run = contract::app().run_spinel_slim(&[], contract::ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("method assignment operands passed"), "{}", run.stdout);
}
