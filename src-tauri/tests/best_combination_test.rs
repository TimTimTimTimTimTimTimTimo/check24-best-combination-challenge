use best_combination_lib::*;

#[test]
fn test_best_combination_single() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_single(&data), @r###"
    game_count: 78
    orphan_count: 1
    best_combination:
      package_ids:
        - 0
    best_combination_properties:
      live_coverage: 77
      high_coverage: 78
      total_coverage: 78
      price: 6000
    "###);
}

#[test]
fn test_best_combination_multi_1() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_1(&data), @r###"
    game_count: 208
    orphan_count: 3
    best_combination:
      package_ids:
        - 0
        - 29
    best_combination_properties:
      live_coverage: 205
      high_coverage: 208
      total_coverage: 208
      price: 6992
    "###);
}

#[test]
fn test_best_combination_all() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_all(&data), @r###"
    game_count: 5661
    orphan_count: 3210
    best_combination:
      package_ids:
        - 0
        - 29
        - 35
        - 17
        - 24
        - 21
    best_combination_properties:
      live_coverage: 5228
      high_coverage: 5661
      total_coverage: 5661
      price: 8192
    "###);
}

#[test]
fn test_best_combination_multi_2() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_2(&data), @r###"
    game_count: 116
    orphan_count: 11
    best_combination:
      package_ids:
        - 16
        - 35
        - 24
    best_combination_properties:
      live_coverage: 116
      high_coverage: 116
      total_coverage: 116
      price: 3199
    "###);
}
