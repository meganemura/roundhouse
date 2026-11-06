//! The interpreted lane of `tests/support/column_or_assign_callback.rs`:
//! a method a symbol callback names fills a NOT NULL string column with
//! `||=`, as the block form of the same callback does. Kept out of
//! tests/emit_and_run.rs so concurrent appends there do not conflict.

#[path = "support/column_or_assign_callback.rs"]
mod column_or_assign_callback;
#[path = "support/emit_and_run.rs"]
mod emit_and_run;

use column_or_assign_callback::{APPLICATION_RECORD, ASSERTIONS, ROOM, SCHEMA};

#[test]
fn a_symbol_callback_or_assign_fills_an_unset_string_column_on_ruby() {
    let run = emit_and_run::empty_app()
        .write("db/schema.rb", SCHEMA)
        .write("app/models/application_record.rb", APPLICATION_RECORD)
        .write(
            "config/routes.rb",
            "Rails.application.routes.draw do\nend\n",
        )
        .write("app/models/room.rb", ROOM)
        .run_ruby(ASSERTIONS);
    run.assert_passes();
    assert!(
        run.stdout
            .contains("column or-assign callback contract passed"),
        "{}",
        run.stdout
    );
}
