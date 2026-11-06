//! `count(column)` compiled natively; the Ruby-lane twin is
//! `tests/relation_count_column.rs`.

use super::relation_count_column_contract as contract;

#[test]
#[ignore = "requires the Spinel toolchain, run in its CI lane"]
fn count_of_a_column_skips_nulls_and_honors_distinct_natively() {
    contract::overlay()
        .run_spinel_with_db(contract::SCRIPT)
        .assert_passes();
}
