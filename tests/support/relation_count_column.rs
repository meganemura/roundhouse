//! One `count(column)` contract shared by the interpreted and native
//! lanes. See `tests/relation_count_column.rs` for what it covers.

pub fn overlay() -> super::emit_and_run::Overlay {
    super::emit_and_run::empty_app()
        .write(
            "app/models/application_record.rb",
            "class ApplicationRecord < ActiveRecord::Base\n  self.abstract_class = true\nend\n",
        )
        .write(
            "app/models/gadget.rb",
            r#"class Gadget < ApplicationRecord
  has_many :widgets

  def distinct_scores
    widgets.count("distinct score")
  end
end
"#,
        )
        .write(
            "app/models/widget.rb",
            r#"class Widget < ApplicationRecord
  belongs_to :gadget, optional: true

  def self.scored
    count(:score)
  end

  def self.kinds
    distinct.count(:kind)
  end

  def self.scored_in(kind)
    where(kind: kind).count(:score)
  end

  def self.scored_outside(kind)
    where.not(kind: kind).count(:score)
  end

  def self.scored_through(kind)
    rel = Widget.where(kind: kind)
    rel.count(:score)
  end

  def self.rows
    count(:all)
  end

  def self.scored_by_name
    Widget.all.count("score")
  end

  def self.rows_by_key
    [count(:id), count("*"), where(kind: "b").count(:all)]
  end

  def self.scored_per_kind
    group(:kind).count(:score)
  end

  def self.scored_per_kind_through_variable
    rel = Widget.group(:kind)
    rel.count(:score)
  end

  def self.plucked_twos
    Widget.all.pluck(:score).count(2)
  end

  def self.mapped_bs
    Widget.where(kind: "b").map(&:kind).count("b")
  end
end
"#,
        )
        .write(
            "db/schema.rb",
            r#"ActiveRecord::Schema.define do
  create_table "gadgets", force: :cascade do |t|
    t.string "name"
  end
  create_table "widgets", force: :cascade do |t|
    t.integer "gadget_id"
    t.string "kind"
    t.integer "score"
  end
end
"#,
        )
}

pub const SCRIPT: &str = r#"
g = Gadget.create!(name: "g")
[["a", 1], ["b", nil], ["b", 2], ["c", 2]].each { |k, s| Widget.create!(gadget: g, kind: k, score: s) }
Widget.create!(kind: "c", score: nil)
got = [Widget.scored, Widget.kinds, Widget.scored_in("b"), Widget.scored_outside("b"), Widget.scored_through("b"), Widget.rows, Widget.scored_by_name, g.distinct_scores]
raise "column counts: #{got.inspect}" unless got == [3, 3, 1, 2, 1, 5, 3, 2]
raise "row counts: #{Widget.rows_by_key.inspect}" unless Widget.rows_by_key == [5, 5, 2]
raise "grouped column count: #{Widget.scored_per_kind.inspect}" unless Widget.scored_per_kind == { "a" => 1, "b" => 1, "c" => 1 }
got = [Widget.plucked_twos, Widget.mapped_bs]
raise "Array counts: #{got.inspect}" unless got == [2, 2]
# Rails answers the Hash here too; the runtime raises rather than answer
# the number of groups.
begin
  n = Widget.scored_per_kind_through_variable
  raise "grouped count through a variable answered #{n.inspect}"
rescue ArgumentError
end
puts "column counts passed"
"#;
