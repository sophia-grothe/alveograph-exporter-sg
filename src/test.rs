use crate::config_store::ConfigStore;

/// Sample lines from a made-up file, to be used for testing.
pub fn sample_file_lines() -> Vec<String> {
    vec![
        "wigejewg ignore eikgubeg",
        "Test name\t:\tSample001-1234567",
        "iwuefjwef eoiwnfgw oewifnwe",
        "ewgonwegw	ewiugbwegiw	ewoiugnweg",
        "wejignwe	wefoinwg	wkjegngweg",
        "",
        "weigunweg woe fwef	oignweognw",
        "Standard\t : \tAverage",
        "P\t1",
        "L\t2",
        "G\t3",
        "wiuuebwfw fwefwef",
        "oigwefnw woeifnwf",
        "12345671",
    ].iter().map(|s| s.to_string()).collect()
}//end sample_file_lines

/// Sample configuration settings, to be used with
/// sample file from crate::test::sample_file_lines().
pub fn sample_config() -> ConfigStore {
    ConfigStore {
        read_start_header: "Standard\t : \tAverage".to_string(),
        read_start_idx: 7,
        read_row_headers: vec!["P","L","G"].iter().map(|s| s.to_string()).collect(),
        read_max_rows: 3,
        read_start_mode: crate::config_store::ReadStartMode::Header,
        read_row_mode: crate::config_store::ReadRowMode::Header,
        row_order_preference: vec!["G","L","P"].iter().map(|s| s.to_string()).collect(),
        read_row_split_char: "\t".to_string(),
        read_test_name_prefix: "Test name\t:\t".to_string(),
    }//end struct construction
}//end sample_config()

/// Test 1 for crate::data::get_test_name_from_lines()
#[test]
pub fn data_get_test_name_from_lines1() {
    let file_lines = sample_file_lines();
    let config = sample_config();
    let test_name = crate::data::get_test_name_from_lines(
        &file_lines,
        &config
    ).unwrap();
    assert_eq!(test_name, "Sample001-1234567".to_string());
}//end data_get_test_name_from_lines1()

/// Test 1 for crate::data::get_header_idx_from_lines()
#[test]
pub fn data_get_header_idx_from_lines1() {
    let file_lines = sample_file_lines();
    let config = sample_config();
    let header_idx = crate::data::get_header_idx_from_lines(
        "sample-filename",
        &file_lines,
        &config
    ).unwrap();
    assert_eq!(header_idx, 7);
}//end data_get_header_idx_from_lines1

// /// Test 1 for crate::data::read_data_from_file()
// #[test]
// pub fn data_read_data_from_file() {
//     let file_lines = sample_file_lines();
//     let config = sample_config();
//     let (data, errs) = crate::data::read_data_from_file(
//         "sample-filename",
//         &file_lines.join("\n"),
//         &config
//     ).unwrap();
//     assert!(errs.len() == 0);
//     let correct_data = crate::data::Data::new1(
//         "Sample001-1234567".to_string(),
//         vec![
//             crate::data::Row::new("G".to_string(),3.),
//             crate::data::Row::new("L".to_string(),2.),
//             crate::data::Row::new("P".to_string(),1.),
//         ],
//     );
//     assert_eq!(data, correct_data);
// }//end data_read_data_from_file()

/// Test 1 for crate::data::sort_row_data()
#[test]
pub fn data_sort_row_data1() {
    let row_data = vec![
        crate::data::Row::new("P".to_string(),1.),
        crate::data::Row::new("H2O".to_string(),4.),
        crate::data::Row::new("L".to_string(),2.),
        crate::data::Row::new("G".to_string(),3.),
    ];
    let correct_sorted_row_data = vec![
        crate::data::Row::new("G".to_string(),3.),
        crate::data::Row::new("L".to_string(),2.),
        crate::data::Row::new("P".to_string(),1.),
        crate::data::Row::new("H2O".to_string(),4.),
    ];
    let sorted_row_data = crate::data::sort_row_data(
        row_data,
        &sample_config()
    );
    assert_eq!(correct_sorted_row_data,sorted_row_data);
}//end data_sort_row_data1
