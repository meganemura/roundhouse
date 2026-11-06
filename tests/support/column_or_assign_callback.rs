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

/// A method a symbol callback names gets the same rewrite as a block
/// body. The shape is a real app's: `before_validation
/// :assign_join_code, on: :create` with a `loop` that retries until the
/// generated code is free. Without the rewrite every room saved `""`,
/// and the second insert hit the unique index.
pub const ROOM_SCHEMA: &str = r#"ActiveRecord::Schema.define do
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

pub const ROOM_ASSERTIONS: &str = r#"first = Room.create!(name: "First")
raise "first join_code #{first.join_code.inspect}, want \"R1\"" unless first.join_code == "R1"
second = Room.create!(name: "Second")
raise "second join_code #{second.join_code.inspect}, want \"R2\"" unless second.join_code == "R2"
given = Room.create!(name: "Given", join_code: "MINE")
raise "given join_code #{given.join_code.inspect}, want \"MINE\"" unless given.join_code == "MINE"
reloaded = Room.find(second.id).join_code
raise "reloaded join_code #{reloaded.inspect}, want \"R2\"" unless reloaded == "R2"
puts "symbol callback join code contract passed"
"#;

/// The symbol form keeps a stored `""` and fills a nil as the block form
/// does. The named method can also be called for its value, and `||=`
/// evaluates to the column whether it assigned or not.
pub const TICKET_SCHEMA: &str = r#"ActiveRecord::Schema.define do
  create_table "tickets", force: :cascade do |t|
    t.string "name", null: false
    t.string "code", null: false
  end
end
"#;

pub const TICKET: &str = r#"class Ticket < ApplicationRecord
  before_validation :fill_code

  def shown_code
    fill_code
  end

  private
    def fill_code
      self.code ||= "FILLED"
    end
end
"#;

pub const TICKET_ASSERTIONS: &str = r##"def check(label, got, want)
  raise "#{label}: #{got.inspect}, want #{want.inspect}" unless got == want
end

t = Ticket.new(name: "new")
t.valid?
check("unset on a new record", t.code, "FILLED")
t = Ticket.new(name: "given", code: "x")
t.valid?
check("given on a new record", t.code, "x")
t = Ticket.create!(name: "kept", code: "x")
t.code = ""
t.save!
check("\"\" assigned on a persisted record", Ticket.find(t.id).code, "")
loaded = Ticket.find(t.id)
loaded.valid?
check("loaded with a stored \"\"", loaded.code, "")
loaded.code = nil
loaded.valid?
check("nil assigned on a loaded record", loaded.code, "FILLED")
clean = Ticket.find(Ticket.create!(name: "clean", code: "x").id)
clean.valid?
check("a kept column reads as unchanged", clean.changed?, false)
spaced = Ticket.new(name: "spaces", code: "  ")
spaced.valid?
check("whitespace on a new record", spaced.code, "  ")

check("value when it fills", Ticket.new(name: "memo").shown_code, "FILLED")
check("value of a set column", Ticket.new(name: "memo", code: "x").shown_code, "x")
check("value of a stored \"\"", Ticket.find(t.id).shown_code, "")
puts "symbol callback or-assign contract passed"
"##;

/// Ruby's `||=` evaluates to the column whether it assigned or not, and
/// a method a symbol callback names can use that value anywhere: a
/// local, a returned value, another writer's argument, a method's
/// argument, a condition, or an operand. Each check reads the column
/// after the use too, so a rewrite that leaks the operator around it
/// into the assignment shows up as a wrong column.
pub const CAPTURE_SCHEMA: &str = r#"ActiveRecord::Schema.define do
  create_table "captures", force: :cascade do |t|
    t.string "name", null: false
    t.string "code", null: false
  end
end
"#;

pub const CAPTURE: &str = r#"class Capture < ApplicationRecord
  before_validation :captured, :returned, :copied, :passed, :tested, :chosen, :compared, :joined

  def captured
    y = (self.code ||= "Y")
    y.upcase
  end

  def returned
    return (self.code ||= "R") if name == "ret"
    "other"
  end

  def copied
    self.name = (self.code ||= "N")
    name
  end

  def passed
    wrapped(self.code ||= "P")
  end

  def tested
    if (self.code ||= "I") == "I"
      "filled"
    else
      "kept"
    end
  end

  def chosen
    (self.code ||= "T") == "T" ? "filled" : "kept"
  end

  def compared
    (self.code ||= "E") == "E"
  end

  def joined
    (self.code ||= "ab") + "!"
  end

  def wrapped(text)
    "<#{text}>"
  end
end
"#;

pub const CAPTURE_ASSERTIONS: &str = r##"def check(label, got, want)
  raise "#{label}: #{got.inspect}, want #{want.inspect}" unless got == want
end

c = Capture.new(name: "a")
check("a local, filled", c.captured, "Y")
check("a local, filled: the column", c.code, "Y")
c = Capture.new(name: "a", code: "q")
check("a local, kept", c.captured, "Q")
check("a local, kept: the column", c.code, "q")
c = Capture.new(name: "ret")
check("a returned value, filled", c.returned, "R")
check("a returned value, filled: the column", c.code, "R")
c = Capture.new(name: "ret", code: "q")
check("a returned value, kept", c.returned, "q")
check("a returned value, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("a writer's argument, filled", c.copied, "N")
check("a writer's argument, filled: the column", c.code, "N")
c = Capture.new(name: "a", code: "q")
check("a writer's argument, kept", c.copied, "q")
check("a writer's argument, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("a method's argument, filled", c.passed, "<P>")
check("a method's argument, filled: the column", c.code, "P")
c = Capture.new(name: "a", code: "q")
check("a method's argument, kept", c.passed, "<q>")
check("a method's argument, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("an if's condition, filled", c.tested, "filled")
check("an if's condition, filled: the column", c.code, "I")
c = Capture.new(name: "a", code: "q")
check("an if's condition, kept", c.tested, "kept")
check("an if's condition, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("a ternary's condition, filled", c.chosen, "filled")
check("a ternary's condition, filled: the column", c.code, "T")
c = Capture.new(name: "a", code: "q")
check("a ternary's condition, kept", c.chosen, "kept")
check("a ternary's condition, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("an operand of ==, filled", c.compared, true)
check("an operand of ==, filled: the column", c.code, "E")
c = Capture.new(name: "a", code: "q")
check("an operand of ==, kept", c.compared, false)
check("an operand of ==, kept: the column", c.code, "q")
c = Capture.new(name: "a")
check("an operand of +, filled", c.joined, "ab!")
check("an operand of +, filled: the column", c.code, "ab")
c = Capture.new(name: "a", code: "q")
check("an operand of +, kept", c.joined, "q!")
check("an operand of +, kept: the column", c.code, "q")
stored = Capture.create!(name: "a", code: "q")
stored.code = ""
stored.save!
c = Capture.find(stored.id)
check("an operand of +, a stored \"\" kept", c.joined, "!")
check("an operand of +, a stored \"\" kept: the column", c.code, "")
puts "symbol callback or-assign value contract passed"
"##;
