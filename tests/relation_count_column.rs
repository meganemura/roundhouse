//! `count(column)` is SQL `COUNT(column)`, as in Rails: rows whose
//! column is NULL are not counted, a `distinct` relation counts distinct
//! values, `:all` counts every row, and a String is a SQL expression
//! (`stories.count("distinct user_id")` is how lobsters counts a
//! domain's submitters). Each method below reaches `count` through a
//! different path: the model class, a chain the compiler folds into SQL,
//! a chain left to the runtime Relation, and a has_many association.
//! `count(:id)` and `count("*")` count every row like `count(:all)`,
//! `group(:kind).count(:score)` answers a Hash of per-group column
//! counts, and `count(obj)` on a plucked or mapped Array stays
//! `Array#count`. The expected numbers are what Rails 8.1 answers for
//! the same schema, rows and methods. One case differs on purpose:
//! `count(:score)` on a grouped relation held in a variable raises,
//! where Rails answers the Hash, rather than answer the number of groups.

#[path = "support/emit_and_run.rs"]
mod emit_and_run;
#[path = "support/relation_count_column.rs"]
mod contract;

#[test]
fn count_of_a_column_skips_nulls_and_honors_distinct() {
    contract::overlay().run_ruby(contract::SCRIPT).assert_passes();
}
