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

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_symbol_callback_or_assign_generates_a_join_code_natively() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::ROOM_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/room.rb", contract::ROOM)
        .run_spinel_with_db(contract::ROOM_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback join code contract passed"), "{}", run.stdout);
}

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_symbol_callback_or_assign_fills_and_returns_the_column_natively() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::TICKET_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/ticket.rb", contract::TICKET)
        .run_spinel_with_db(contract::TICKET_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback or-assign contract passed"), "{}", run.stdout);
}

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn a_symbol_callback_or_assign_yields_the_column_wherever_it_stands_natively() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::CAPTURE_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/capture.rb", contract::CAPTURE)
        .run_spinel_with_db(contract::CAPTURE_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback or-assign value contract passed"), "{}", run.stdout);
}
