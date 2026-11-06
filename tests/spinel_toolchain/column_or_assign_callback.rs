use super::{column_or_assign_callback_contract as contract, emit_and_run};

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_block_callback_or_assign_keeps_a_stored_blank_string_column_natively() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::BLOCK_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/badge.rb", contract::BADGE)
        .write("app/models/stamp.rb", contract::STAMP)
        .run_spinel_with_db(contract::BLOCK_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("block or-assign contract passed"), "{}", run.stdout);
}
