use best_combination_lib::*;

#[test]
fn test_best_combination_single() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_single(&data), @r###"
        package_ids:
          - 0
        live_coverage: 77
        highlights_coverage: 78
        total_coverage: 78
        total_price: 6000
        "###);
}

#[test]
fn test_best_combination_multi_1() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_1(&data), @r###"
    package_ids:
      - 0
      - 29
    live_coverage: 208
    highlights_coverage: 211
    total_coverage: 211
    total_price: 6992
    "###);
}

#[test]
fn test_best_combination_all() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_all(&data), @r###"
    package_ids:
      - 0
      - 29
      - 35
      - 17
      - 24
      - 8
    live_coverage: 5272
    highlights_coverage: 6560
    total_coverage: 6594
    total_price: 8192
    "###);
}

#[test]
fn test_best_combination_multi_2() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_2(&data), @r###"
    package_ids:
      - 16
      - 35
      - 24
    live_coverage: 116
    highlights_coverage: 116
    total_coverage: 116
    total_price: 3199
    "###);
}
