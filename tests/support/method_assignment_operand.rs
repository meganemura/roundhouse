//! A method assignment (`self.code = v`, `h[k] = v`) as the operand of
//! an infix operator, shared by the Ruby lane and the Spinel lane. An
//! assignment binds looser than every operator, so its parentheses carry
//! the meaning: `(self.code = "x") == "x"` printed bare is
//! `self.code = ("x" == "x")`.

use super::emit_and_run;

pub const TALLY: &str = r##"class Tally
  attr_accessor :code

  def compared
    [(self.code = "x") == "x", code]
  end

  def joined
    [(self.code = "ab") + "!", code]
  end

  def chosen
    [((self.code = "T") == "T" ? "yes" : "no"), code]
  end

  def tested
    if (self.code = "I") == "I"
      ["yes", code]
    else
      ["no", code]
    end
  end

  def indexed
    counts = {}
    [(counts[:a] = 2) * 3, counts[:a]]
  end
end
"##;

/// The expected values are what CRuby returns for the class as written.
pub const ASSERTIONS: &str = r##"def check(label, got, want)
  raise "#{label}: #{got.inspect}, want #{want.inspect}" unless got == want
end

check("an operand of ==", Tally.new.compared, [true, "x"])
check("an operand of +", Tally.new.joined, ["ab!", "ab"])
check("a ternary's condition", Tally.new.chosen, ["yes", "T"])
check("an if's condition", Tally.new.tested, ["yes", "I"])
check("an index assignment", Tally.new.indexed, [6, 2])
puts "method assignment operands passed"
"##;

#[allow(dead_code)]
pub fn app() -> emit_and_run::Overlay {
    emit_and_run::real_blog().write("app/models/tally.rb", TALLY)
}
