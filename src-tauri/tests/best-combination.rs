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
