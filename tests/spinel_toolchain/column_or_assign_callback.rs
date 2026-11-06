use super::{column_or_assign_callback_contract as contract, emit_and_run};

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_symbol_callback_or_assign_fills_an_unset_string_column_natively() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::SCHEMA)
        .write(
            "app/models/application_record.rb",
            contract::APPLICATION_RECORD,
        )
        .write(
            "config/routes.rb",
            "Rails.application.routes.draw do\nend\n",
        )
        .write("app/models/room.rb", contract::ROOM)
        .run_spinel_with_db(contract::ASSERTIONS);
    run.assert_passes();
    assert!(
        run.stdout
            .contains("column or-assign callback contract passed"),
        "{}",
        run.stdout
    );
}
