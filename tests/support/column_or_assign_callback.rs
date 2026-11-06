//! Contracts for `self.<col> ||= value` on a NOT NULL string column in
//! a model callback, shared by the interpreted and native lanes.
//!
//! Rails starts an unset column as nil, so `||=` fills it. This runtime
//! starts a NOT NULL string slot as `""`, which is truthy, so the
//! callback rewrites the `||=` to also fill a blank column. That reading
//! of `""` as unset holds only for a record that was never saved or
//! loaded. Every expected value is what Rails 8.1.4 returns for the
//! same models.

pub const APPLICATION_RECORD: &str =
    "class ApplicationRecord < ActiveRecord::Base\n  primary_abstract_class\nend\n";

/// A block callback and an `after_initialize` block: a persisted row's
/// `""` is a stored value that Rails' `||=` keeps, and a nil the app
/// assigns still fills, as Ruby's own `||=` does.
pub const BLOCK_SCHEMA: &str = r#"ActiveRecord::Schema.define do
  create_table "badges", force: :cascade do |t|
    t.string "name", null: false
    t.string "code", null: false
  end
  create_table "stamps", force: :cascade do |t|
    t.string "name", null: false
    t.string "code", null: false
  end
end
"#;

pub const BADGE: &str = r#"class Badge < ApplicationRecord
  before_validation do
    self.code ||= "FILLED"
  end
end
"#;

pub const STAMP: &str = r#"class Stamp < ApplicationRecord
  after_initialize do
    self.code ||= "FILLED"
  end
end
"#;

pub const BLOCK_ASSERTIONS: &str = r##"def check(label, got, want)
  raise "#{label}: code #{got.inspect}, want #{want.inspect}" unless got == want
end

b = Badge.new(name: "new")
b.valid?
check("badge unset on a new record", b.code, "FILLED")
b = Badge.new(name: "given", code: "x")
b.valid?
check("badge given on a new record", b.code, "x")
b = Badge.create!(name: "kept", code: "x")
b.code = ""
b.save!
check("badge \"\" assigned on a persisted record", Badge.find(b.id).code, "")
loaded = Badge.find(b.id)
loaded.valid?
check("badge loaded with a stored \"\"", loaded.code, "")
loaded.code = nil
loaded.valid?
check("badge nil assigned on a loaded record", loaded.code, "FILLED")

check("stamp unset on a new record", Stamp.new(name: "new").code, "FILLED")
check("stamp given on a new record", Stamp.new(name: "given", code: "x").code, "x")
s = Stamp.create!(name: "kept", code: "x")
s.code = ""
s.save!
check("stamp loaded with a stored \"\"", Stamp.find(s.id).code, "")
puts "block or-assign contract passed"
"##;
