use super::method_assignment_operand_contract as contract;

#[test]
fn a_method_assignment_operand_of_an_infix_operator_keeps_its_parentheses() {
    let run = contract::app().run_ruby(contract::ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("method assignment operands passed"), "{}", run.stdout);
}
