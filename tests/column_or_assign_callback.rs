//! The interpreted lane of `tests/support/column_or_assign_callback.rs`:
//! `self.<col> ||= value` on a NOT NULL string column in a callback.
//! Kept out of tests/emit_and_run.rs so concurrent appends there do not
//! conflict.

#[path = "support/column_or_assign_callback.rs"]
mod column_or_assign_callback;
#[path = "support/emit_and_run.rs"]
mod emit_and_run;

use column_or_assign_callback as contract;

#[test]
fn a_block_callback_or_assign_keeps_a_stored_blank_string_column_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::BLOCK_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/badge.rb", contract::BADGE)
        .write("app/models/stamp.rb", contract::STAMP)
        .run_ruby(contract::BLOCK_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("block or-assign contract passed"), "{}", run.stdout);
}

#[test]
fn a_symbol_callback_or_assign_generates_a_join_code_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::ROOM_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/room.rb", contract::ROOM)
        .run_ruby(contract::ROOM_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback join code contract passed"), "{}", run.stdout);
}

#[test]
fn a_symbol_callback_or_assign_fills_and_returns_the_column_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::TICKET_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/ticket.rb", contract::TICKET)
        .run_ruby(contract::TICKET_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback or-assign contract passed"), "{}", run.stdout);
}

#[test]
fn a_symbol_callback_or_assign_yields_the_column_wherever_it_stands_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", contract::CAPTURE_SCHEMA)
        .write("app/models/application_record.rb", contract::APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/capture.rb", contract::CAPTURE)
        .run_ruby(contract::CAPTURE_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("symbol callback or-assign value contract passed"), "{}", run.stdout);
}
