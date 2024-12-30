use best_combination_core::*;

#[test]
fn test_best_combination_single() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_single(&data), @r###"
    game_count: 78
    orphan_count: 1
    cheapest_combination:
      package_ids:
        - 8
        - 15
      coverages:
        high_coverage: 78
        live_coverage: 0
        some_coverage: 78
        full_coverage: 0
      price: 699
    smallest_combination:
      package_ids:
        - 0
      coverages:
        high_coverage: 78
        live_coverage: 77
        some_coverage: 78
        full_coverage: 77
      price: 6000
    single_combinations:
      - package_ids:
          - 0
        coverages:
          high_coverage: 78
          live_coverage: 77
          some_coverage: 78
          full_coverage: 77
        price: 6000
      - package_ids:
          - 1
        coverages:
          high_coverage: 76
          live_coverage: 0
          some_coverage: 76
          full_coverage: 0
        price: 0
      - package_ids:
          - 2
        coverages:
          high_coverage: 69
          live_coverage: 0
          some_coverage: 69
          full_coverage: 0
        price: 0
      - package_ids:
          - 3
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 583
      - package_ids:
          - 4
        coverages:
          high_coverage: 14
          live_coverage: 9
          some_coverage: 14
          full_coverage: 9
        price: 2999
      - package_ids:
          - 5
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 0
      - package_ids:
          - 6
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 749
      - package_ids:
          - 7
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 1249
      - package_ids:
          - 8
        coverages:
          high_coverage: 76
          live_coverage: 0
          some_coverage: 76
          full_coverage: 0
        price: 0
      - package_ids:
          - 9
        coverages:
          high_coverage: 76
          live_coverage: 1
          some_coverage: 76
          full_coverage: 1
        price: 749
      - package_ids:
          - 10
        coverages:
          high_coverage: 21
          live_coverage: 11
          some_coverage: 21
          full_coverage: 11
        price: 3499
      - package_ids:
          - 11
        coverages:
          high_coverage: 12
          live_coverage: 7
          some_coverage: 12
          full_coverage: 7
        price: 3000
      - package_ids:
          - 12
        coverages:
          high_coverage: 18
          live_coverage: 0
          some_coverage: 18
          full_coverage: 0
        price: 649
      - package_ids:
          - 13
        coverages:
          high_coverage: 2
          live_coverage: 2
          some_coverage: 2
          full_coverage: 2
        price: 2500
      - package_ids:
          - 14
        coverages:
          high_coverage: 14
          live_coverage: 9
          some_coverage: 14
          full_coverage: 9
        price: 3500
      - package_ids:
          - 15
        coverages:
          high_coverage: 9
          live_coverage: 0
          some_coverage: 9
          full_coverage: 0
        price: 699
      - package_ids:
          - 16
        coverages:
          high_coverage: 9
          live_coverage: 0
          some_coverage: 9
          full_coverage: 0
        price: 1999
      - package_ids:
          - 17
        coverages:
          high_coverage: 7
          live_coverage: 0
          some_coverage: 7
          full_coverage: 0
        price: 0
      - package_ids:
          - 18
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 1500
      - package_ids:
          - 19
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 1000
      - package_ids:
          - 20
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1295
      - package_ids:
          - 21
        coverages:
          high_coverage: 69
          live_coverage: 1
          some_coverage: 69
          full_coverage: 1
        price: 0
      - package_ids:
          - 22
        coverages:
          high_coverage: 5
          live_coverage: 0
          some_coverage: 5
          full_coverage: 0
        price: 0
      - package_ids:
          - 23
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 24
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 375
      - package_ids:
          - 25
        coverages:
          high_coverage: 7
          live_coverage: 0
          some_coverage: 7
          full_coverage: 0
        price: 1239
      - package_ids:
          - 26
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 27
        coverages:
          high_coverage: 69
          live_coverage: 0
          some_coverage: 69
          full_coverage: 0
        price: 199
      - package_ids:
          - 28
        coverages:
          high_coverage: 76
          live_coverage: 1
          some_coverage: 76
          full_coverage: 1
        price: 899
      - package_ids:
          - 29
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 992
      - package_ids:
          - 30
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 1299
      - package_ids:
          - 31
        coverages:
          high_coverage: 19
          live_coverage: 11
          some_coverage: 19
          full_coverage: 11
        price: 4499
      - package_ids:
          - 32
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 699
      - package_ids:
          - 33
        coverages:
          high_coverage: 7
          live_coverage: 0
          some_coverage: 7
          full_coverage: 0
        price: 1398
      - package_ids:
          - 34
        coverages:
          high_coverage: 19
          live_coverage: 0
          some_coverage: 19
          full_coverage: 0
        price: 2332
      - package_ids:
          - 35
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 825
      - package_ids:
          - 36
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1499
    "###);
}

#[test]
fn test_best_combination_multi_1() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_1(&data), @r###"
    game_count: 208
    orphan_count: 3
    cheapest_combination:
      package_ids:
        - 8
        - 29
        - 16
        - 22
      coverages:
        high_coverage: 202
        live_coverage: 122
        some_coverage: 208
        full_coverage: 116
      price: 2991
    smallest_combination:
      package_ids:
        - 0
        - 29
      coverages:
        high_coverage: 208
        live_coverage: 205
        some_coverage: 208
        full_coverage: 205
      price: 6992
    single_combinations:
      - package_ids:
          - 0
        coverages:
          high_coverage: 133
          live_coverage: 130
          some_coverage: 133
          full_coverage: 130
        price: 6000
      - package_ids:
          - 1
        coverages:
          high_coverage: 89
          live_coverage: 2
          some_coverage: 89
          full_coverage: 2
        price: 0
      - package_ids:
          - 2
        coverages:
          high_coverage: 74
          live_coverage: 3
          some_coverage: 74
          full_coverage: 3
        price: 0
      - package_ids:
          - 3
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 583
      - package_ids:
          - 4
        coverages:
          high_coverage: 19
          live_coverage: 9
          some_coverage: 19
          full_coverage: 9
        price: 2999
      - package_ids:
          - 5
        coverages:
          high_coverage: 27
          live_coverage: 6
          some_coverage: 33
          full_coverage: 0
        price: 0
      - package_ids:
          - 6
        coverages:
          high_coverage: 27
          live_coverage: 6
          some_coverage: 33
          full_coverage: 0
        price: 749
      - package_ids:
          - 7
        coverages:
          high_coverage: 27
          live_coverage: 6
          some_coverage: 33
          full_coverage: 0
        price: 1249
      - package_ids:
          - 8
        coverages:
          high_coverage: 84
          live_coverage: 5
          some_coverage: 89
          full_coverage: 0
        price: 0
      - package_ids:
          - 9
        coverages:
          high_coverage: 84
          live_coverage: 9
          some_coverage: 90
          full_coverage: 3
        price: 749
      - package_ids:
          - 10
        coverages:
          high_coverage: 71
          live_coverage: 64
          some_coverage: 76
          full_coverage: 59
        price: 3499
      - package_ids:
          - 11
        coverages:
          high_coverage: 12
          live_coverage: 12
          some_coverage: 17
          full_coverage: 7
        price: 3000
      - package_ids:
          - 12
        coverages:
          high_coverage: 26
          live_coverage: 1
          some_coverage: 27
          full_coverage: 0
        price: 649
      - package_ids:
          - 13
        coverages:
          high_coverage: 2
          live_coverage: 2
          some_coverage: 2
          full_coverage: 2
        price: 2500
      - package_ids:
          - 14
        coverages:
          high_coverage: 14
          live_coverage: 9
          some_coverage: 14
          full_coverage: 9
        price: 3500
      - package_ids:
          - 15
        coverages:
          high_coverage: 20
          live_coverage: 3
          some_coverage: 20
          full_coverage: 3
        price: 699
      - package_ids:
          - 16
        coverages:
          high_coverage: 58
          live_coverage: 41
          some_coverage: 58
          full_coverage: 41
        price: 1999
      - package_ids:
          - 17
        coverages:
          high_coverage: 15
          live_coverage: 0
          some_coverage: 15
          full_coverage: 0
        price: 0
      - package_ids:
          - 18
        coverages:
          high_coverage: 28
          live_coverage: 1
          some_coverage: 28
          full_coverage: 1
        price: 1500
      - package_ids:
          - 19
        coverages:
          high_coverage: 28
          live_coverage: 1
          some_coverage: 28
          full_coverage: 1
        price: 1000
      - package_ids:
          - 20
        coverages:
          high_coverage: 3
          live_coverage: 3
          some_coverage: 3
          full_coverage: 3
        price: 1295
      - package_ids:
          - 21
        coverages:
          high_coverage: 69
          live_coverage: 1
          some_coverage: 69
          full_coverage: 1
        price: 0
      - package_ids:
          - 22
        coverages:
          high_coverage: 5
          live_coverage: 1
          some_coverage: 6
          full_coverage: 0
        price: 0
      - package_ids:
          - 23
        coverages:
          high_coverage: 3
          live_coverage: 0
          some_coverage: 3
          full_coverage: 0
        price: 0
      - package_ids:
          - 24
        coverages:
          high_coverage: 3
          live_coverage: 3
          some_coverage: 3
          full_coverage: 3
        price: 375
      - package_ids:
          - 25
        coverages:
          high_coverage: 18
          live_coverage: 4
          some_coverage: 19
          full_coverage: 3
        price: 1239
      - package_ids:
          - 26
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 27
        coverages:
          high_coverage: 72
          live_coverage: 3
          some_coverage: 72
          full_coverage: 3
        price: 199
      - package_ids:
          - 28
        coverages:
          high_coverage: 87
          live_coverage: 7
          some_coverage: 88
          full_coverage: 6
        price: 899
      - package_ids:
          - 29
        coverages:
          high_coverage: 78
          live_coverage: 78
          some_coverage: 78
          full_coverage: 78
        price: 992
      - package_ids:
          - 30
        coverages:
          high_coverage: 30
          live_coverage: 4
          some_coverage: 31
          full_coverage: 3
        price: 1299
      - package_ids:
          - 31
        coverages:
          high_coverage: 69
          live_coverage: 59
          some_coverage: 69
          full_coverage: 59
        price: 4499
      - package_ids:
          - 32
        coverages:
          high_coverage: 27
          live_coverage: 1
          some_coverage: 28
          full_coverage: 0
        price: 699
      - package_ids:
          - 33
        coverages:
          high_coverage: 18
          live_coverage: 4
          some_coverage: 19
          full_coverage: 3
        price: 1398
      - package_ids:
          - 34
        coverages:
          high_coverage: 30
          live_coverage: 4
          some_coverage: 31
          full_coverage: 3
        price: 2332
      - package_ids:
          - 35
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 825
      - package_ids:
          - 36
        coverages:
          high_coverage: 3
          live_coverage: 3
          some_coverage: 3
          full_coverage: 3
        price: 1499
    "###);
}

#[test]
fn test_best_combination_all() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_all(&data), @r###"
    game_count: 5661
    orphan_count: 3210
    cheapest_combination:
      package_ids:
        - 16
        - 29
        - 13
        - 1
        - 35
        - 24
        - 5
      coverages:
        high_coverage: 5659
        live_coverage: 3417
        some_coverage: 5661
        full_coverage: 3415
      price: 6691
    smallest_combination:
      package_ids:
        - 16
        - 4
        - 29
        - 9
        - 35
        - 24
      coverages:
        high_coverage: 5660
        live_coverage: 3772
        some_coverage: 5661
        full_coverage: 3771
      price: 7939
    single_combinations:
      - package_ids:
          - 0
        coverages:
          high_coverage: 4331
          live_coverage: 3991
          some_coverage: 4331
          full_coverage: 3991
        price: 6000
      - package_ids:
          - 1
        coverages:
          high_coverage: 1420
          live_coverage: 17
          some_coverage: 1420
          full_coverage: 17
        price: 0
      - package_ids:
          - 2
        coverages:
          high_coverage: 1280
          live_coverage: 15
          some_coverage: 1280
          full_coverage: 15
        price: 0
      - package_ids:
          - 3
        coverages:
          high_coverage: 0
          live_coverage: 12
          some_coverage: 12
          full_coverage: 0
        price: 583
      - package_ids:
          - 4
        coverages:
          high_coverage: 1539
          live_coverage: 1115
          some_coverage: 1541
          full_coverage: 1113
        price: 2999
      - package_ids:
          - 5
        coverages:
          high_coverage: 246
          live_coverage: 37
          some_coverage: 282
          full_coverage: 1
        price: 0
      - package_ids:
          - 6
        coverages:
          high_coverage: 246
          live_coverage: 49
          some_coverage: 294
          full_coverage: 1
        price: 749
      - package_ids:
          - 7
        coverages:
          high_coverage: 246
          live_coverage: 49
          some_coverage: 294
          full_coverage: 1
        price: 1249
      - package_ids:
          - 8
        coverages:
          high_coverage: 754
          live_coverage: 34
          some_coverage: 788
          full_coverage: 0
        price: 0
      - package_ids:
          - 9
        coverages:
          high_coverage: 1369
          live_coverage: 40
          some_coverage: 1404
          full_coverage: 5
        price: 749
      - package_ids:
          - 10
        coverages:
          high_coverage: 3060
          live_coverage: 1600
          some_coverage: 3106
          full_coverage: 1554
        price: 3499
      - package_ids:
          - 11
        coverages:
          high_coverage: 724
          live_coverage: 730
          some_coverage: 770
          full_coverage: 684
        price: 3000
      - package_ids:
          - 12
        coverages:
          high_coverage: 244
          live_coverage: 15
          some_coverage: 258
          full_coverage: 1
        price: 649
      - package_ids:
          - 13
        coverages:
          high_coverage: 763
          live_coverage: 763
          some_coverage: 763
          full_coverage: 763
        price: 2500
      - package_ids:
          - 14
        coverages:
          high_coverage: 1487
          live_coverage: 1447
          some_coverage: 1487
          full_coverage: 1447
        price: 3500
      - package_ids:
          - 15
        coverages:
          high_coverage: 1897
          live_coverage: 237
          some_coverage: 1897
          full_coverage: 237
        price: 699
      - package_ids:
          - 16
        coverages:
          high_coverage: 3040
          live_coverage: 1380
          some_coverage: 3040
          full_coverage: 1380
        price: 1999
      - package_ids:
          - 17
        coverages:
          high_coverage: 228
          live_coverage: 0
          some_coverage: 228
          full_coverage: 0
        price: 0
      - package_ids:
          - 18
        coverages:
          high_coverage: 248
          live_coverage: 3
          some_coverage: 248
          full_coverage: 3
        price: 1500
      - package_ids:
          - 19
        coverages:
          high_coverage: 248
          live_coverage: 3
          some_coverage: 248
          full_coverage: 3
        price: 1000
      - package_ids:
          - 20
        coverages:
          high_coverage: 774
          live_coverage: 774
          some_coverage: 774
          full_coverage: 774
        price: 1295
      - package_ids:
          - 21
        coverages:
          high_coverage: 1385
          live_coverage: 5
          some_coverage: 1385
          full_coverage: 5
        price: 0
      - package_ids:
          - 22
        coverages:
          high_coverage: 43
          live_coverage: 3
          some_coverage: 46
          full_coverage: 0
        price: 0
      - package_ids:
          - 23
        coverages:
          high_coverage: 5
          live_coverage: 0
          some_coverage: 5
          full_coverage: 0
        price: 0
      - package_ids:
          - 24
        coverages:
          high_coverage: 10
          live_coverage: 10
          some_coverage: 10
          full_coverage: 10
        price: 375
      - package_ids:
          - 25
        coverages:
          high_coverage: 143
          live_coverage: 7
          some_coverage: 145
          full_coverage: 5
        price: 1239
      - package_ids:
          - 26
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 27
        coverages:
          high_coverage: 1236
          live_coverage: 5
          some_coverage: 1236
          full_coverage: 5
        price: 199
      - package_ids:
          - 28
        coverages:
          high_coverage: 1374
          live_coverage: 11
          some_coverage: 1375
          full_coverage: 10
        price: 899
      - package_ids:
          - 29
        coverages:
          high_coverage: 745
          live_coverage: 745
          some_coverage: 745
          full_coverage: 745
        price: 992
      - package_ids:
          - 30
        coverages:
          high_coverage: 251
          live_coverage: 8
          some_coverage: 253
          full_coverage: 6
        price: 1299
      - package_ids:
          - 31
        coverages:
          high_coverage: 1628
          live_coverage: 1555
          some_coverage: 1628
          full_coverage: 1555
        price: 4499
      - package_ids:
          - 32
        coverages:
          high_coverage: 246
          live_coverage: 3
          some_coverage: 248
          full_coverage: 1
        price: 699
      - package_ids:
          - 33
        coverages:
          high_coverage: 143
          live_coverage: 7
          some_coverage: 145
          full_coverage: 5
        price: 1398
      - package_ids:
          - 34
        coverages:
          high_coverage: 251
          live_coverage: 8
          some_coverage: 253
          full_coverage: 6
        price: 2332
      - package_ids:
          - 35
        coverages:
          high_coverage: 492
          live_coverage: 492
          some_coverage: 492
          full_coverage: 492
        price: 825
      - package_ids:
          - 36
        coverages:
          high_coverage: 8
          live_coverage: 8
          some_coverage: 8
          full_coverage: 8
        price: 1499
    "###);
}

#[test]
fn test_best_combination_multi_2() {
    let data = load_data();
    insta::assert_yaml_snapshot!(best_combination_multi_2(&data), @r###"
    game_count: 116
    orphan_count: 11
    cheapest_combination:
      package_ids:
        - 16
        - 35
        - 24
      coverages:
        high_coverage: 116
        live_coverage: 116
        some_coverage: 116
        full_coverage: 116
      price: 3199
    smallest_combination: ~
    single_combinations:
      - package_ids:
          - 0
        coverages:
          high_coverage: 81
          live_coverage: 81
          some_coverage: 81
          full_coverage: 81
        price: 6000
      - package_ids:
          - 1
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 2
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 3
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 583
      - package_ids:
          - 4
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 2999
      - package_ids:
          - 5
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 6
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 749
      - package_ids:
          - 7
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1249
      - package_ids:
          - 8
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 9
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 749
      - package_ids:
          - 10
        coverages:
          high_coverage: 81
          live_coverage: 81
          some_coverage: 81
          full_coverage: 81
        price: 3499
      - package_ids:
          - 11
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 3000
      - package_ids:
          - 12
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 649
      - package_ids:
          - 13
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 2500
      - package_ids:
          - 14
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 3500
      - package_ids:
          - 15
        coverages:
          high_coverage: 5
          live_coverage: 5
          some_coverage: 5
          full_coverage: 5
        price: 699
      - package_ids:
          - 16
        coverages:
          high_coverage: 81
          live_coverage: 81
          some_coverage: 81
          full_coverage: 81
        price: 1999
      - package_ids:
          - 17
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 18
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1500
      - package_ids:
          - 19
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1000
      - package_ids:
          - 20
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1295
      - package_ids:
          - 21
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 22
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 23
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 24
        coverages:
          high_coverage: 1
          live_coverage: 1
          some_coverage: 1
          full_coverage: 1
        price: 375
      - package_ids:
          - 25
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1239
      - package_ids:
          - 26
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 0
      - package_ids:
          - 27
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 199
      - package_ids:
          - 28
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 899
      - package_ids:
          - 29
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 992
      - package_ids:
          - 30
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1299
      - package_ids:
          - 31
        coverages:
          high_coverage: 81
          live_coverage: 81
          some_coverage: 81
          full_coverage: 81
        price: 4499
      - package_ids:
          - 32
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 699
      - package_ids:
          - 33
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1398
      - package_ids:
          - 34
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 2332
      - package_ids:
          - 35
        coverages:
          high_coverage: 34
          live_coverage: 34
          some_coverage: 34
          full_coverage: 34
        price: 825
      - package_ids:
          - 36
        coverages:
          high_coverage: 0
          live_coverage: 0
          some_coverage: 0
          full_coverage: 0
        price: 1499
    "###);
}
