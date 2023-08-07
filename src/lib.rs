mod types;

#[test]
fn test_valid_employee() {
    let emp = types::structs::Employee::new(
        String::from("hello"),
        String::from("john doe"),
        String::from("math"),
        String::from("jane doe"),
        String::from("Project 1"),
        String::from("math"),
        vec![String::from("untitled book"), String::from("other book")],
        1000,
        5,
        String::from("password"),
    );
    assert!(!emp.is_err());
}
