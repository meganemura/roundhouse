//! One contract for `self.<col> ||= value` on a NOT NULL string column,
//! written in a method a symbol callback names, shared by the
//! interpreted and native lanes.
//!
//! Rails starts an unset column as nil, so `||=` fills it. This runtime
//! starts a NOT NULL string slot as `""`, which is truthy, so a bare
//! `||=` never fires. The block form of a callback already guards the
//! assignment on `blank?`; the symbol form ran the method body as
//! written, and every room saved `""` as its join code. The second
//! insert then hit the unique index. The shape is a real app's:
//! `before_validation :assign_join_code, on: :create` with a `loop`
//! that retries until the generated code is free.

pub const SCHEMA: &str = r#"ActiveRecord::Schema.define do
  create_table "rooms", force: :cascade do |t|
    t.string "name", null: false
    t.string "join_code", null: false
    t.index ["join_code"], name: "index_rooms_on_join_code", unique: true
  end
end
"#;

pub const ROOM: &str = r#"class Room < ApplicationRecord
  before_validation :assign_join_code, on: :create

  def self.generate_join_code
    "R#{count + 1}"
  end

  private
    def assign_join_code
      self.join_code ||= loop do
        code = self.class.generate_join_code
        break code unless self.class.exists?(join_code: code)
      end
    end
end
"#;

pub const APPLICATION_RECORD: &str =
    "class ApplicationRecord < ActiveRecord::Base\n  primary_abstract_class\nend\n";

pub const ASSERTIONS: &str = r#"first = Room.create!(name: "First")
raise "first join_code #{first.join_code.inspect}, want \"R1\"" unless first.join_code == "R1"
second = Room.create!(name: "Second")
raise "second join_code #{second.join_code.inspect}, want \"R2\"" unless second.join_code == "R2"
given = Room.create!(name: "Given", join_code: "MINE")
raise "given join_code #{given.join_code.inspect}, want \"MINE\"" unless given.join_code == "MINE"
reloaded = Room.find(second.id).join_code
raise "reloaded join_code #{reloaded.inspect}, want \"R2\"" unless reloaded == "R2"
puts "column or-assign callback contract passed"
"#;
