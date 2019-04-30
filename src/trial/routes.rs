// pub fn getRoutes() -> [String; 2] {
//     let routes = vec![
//         Routing_Path::new("".to_owned(), "".to_owned() , "".to_owned())
//     ];
// }
pub fn get_routes() -> (String, String, Vec<(String, String, String)>) {
    return ("/trial".to_owned(), "trial".to_owned(), vec![
        ("".to_owned(), "dumb.html".to_owned(), "dumb_page".to_owned()),
    ]);
}