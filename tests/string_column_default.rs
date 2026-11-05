//! A new record's string column.
//!
//! Rails uses the schema default, and nil when the column has none.
//! `null: false` is the database constraint, not the initial value, so
//! `self.title ||=` assigns on a new record. An explicit nil stays nil.
//! A declared default is that value, and a given value wins.

#[path = "support/emit_and_run.rs"]
mod emit_and_run;

const SCHEMA: &str = r#"ActiveRecord::Schema[8.1].define(version: 1) do
  create_table "posts", force: :cascade do |t|
    t.string "title", null: false
    t.string "kind", default: "note", null: false
  end
end
"#;

const MODEL: &str = r#"class Post < ApplicationRecord
  def claim_title
    self.title ||= "assigned"
    title
  end
end
"#;

const SCRIPT: &str = r#"
bare = Post.new
raise "no default: #{bare.title.inspect}" unless bare.title.nil?
raise "schema default: #{bare.kind.inspect}" unless bare.kind == "note"
raise "given value: #{Post.new(title: "x").title.inspect}" unless Post.new(title: "x").title == "x"
raise "explicit nil: #{Post.new(kind: nil).kind.inspect}" unless Post.new(kind: nil).kind.nil?
raise "or-assign: #{bare.claim_title.inspect}" unless bare.claim_title == "assigned"
puts "string defaults"
"#;

fn app() -> emit_and_run::Overlay {
    emit_and_run::empty_app()
        .write(
            "config/application.rb",
            "module StringDefaultProbe\n  class Application < Rails::Application\n  end\nend\n",
        )
        .write("config/routes.rb", "Rails.application.routes.draw do\nend\n")
        .write("db/schema.rb", SCHEMA)
        .write(
            "app/models/application_record.rb",
            "class ApplicationRecord < ActiveRecord::Base\n  self.abstract_class = true\nend\n",
        )
        .write("app/models/post.rb", MODEL)
}

#[test]
fn a_new_records_string_is_nil_without_a_default_and_the_schema_value_with_one() {
    let run = app().run_ruby(SCRIPT);
    run.assert_passes();
    assert!(run.stdout.contains("string defaults"), "{}\n{}", run.stdout, run.stderr);
}
