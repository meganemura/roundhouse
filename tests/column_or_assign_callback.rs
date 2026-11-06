//! The interpreted lane of `tests/support/column_or_assign_callback.rs`:
//! `self.<col> ||= value` on a NOT NULL string column in a callback.
//! Kept out of tests/emit_and_run.rs so concurrent appends there do not
//! conflict.

#[path = "support/column_or_assign_callback.rs"]
mod column_or_assign_callback;
#[path = "support/emit_and_run.rs"]
mod emit_and_run;

use column_or_assign_callback::{APPLICATION_RECORD, BADGE, BLOCK_ASSERTIONS, BLOCK_SCHEMA, STAMP};

#[test]
fn a_block_callback_or_assign_keeps_a_stored_blank_string_column_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", BLOCK_SCHEMA)
        .write("app/models/application_record.rb", APPLICATION_RECORD)
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("app/models/badge.rb", BADGE)
        .write("app/models/stamp.rb", STAMP)
        .run_ruby(BLOCK_ASSERTIONS);
    run.assert_passes();
    assert!(run.stdout.contains("block or-assign contract passed"), "{}", run.stdout);
}
