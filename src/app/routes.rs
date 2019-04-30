pub fn get_routes() -> (String, String, Vec<(String, String, String)>) {
    return ("/".to_owned(), "app".to_owned(), vec![
        ("".to_owned(), "index.html".to_owned(), "home_page".to_owned()),
    ]);
}