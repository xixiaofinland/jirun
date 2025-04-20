// // use jirun::commands::handle_template_with_api;
// use jirun::common::mock_api::MockJiraApi;
//
// #[test]
// fn test_template_handle_skips_duplicates() {
//     let mock_api = MockJiraApi::with_existing_subtasks(&["Task A"]);
//
//     let result = handle_template_with_api(
//         Box::new(mock_api),
//         "PROJ-123".into(),
//         Some("john.doe".into()),
//         true, // dry_run
//     );
//
//     assert!(result.is_ok());
//
//     // You can also assert captured stdout here if needed
// }
