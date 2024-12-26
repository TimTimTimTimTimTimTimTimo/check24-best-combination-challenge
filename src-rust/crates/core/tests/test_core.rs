use best_combination_core::*;

#[test]
fn test_best_combination_single() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_single(&data), @r###"
    game_count: 78
    orphan_count: 1
    best_combination:
      package_ids:
        - 8
        - 15
    best_combination_properties:
      live_coverage: 0
      high_coverage: 78
      total_coverage: 78
      price: 699
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
        - 8
        - 29
        - 16
        - 22
    best_combination_properties:
      live_coverage: 122
      high_coverage: 202
      total_coverage: 208
      price: 2991
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
        - 16
        - 29
        - 13
        - 1
        - 35
        - 24
        - 5
    best_combination_properties:
      live_coverage: 3417
      high_coverage: 5659
      total_coverage: 5661
      price: 6691
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
