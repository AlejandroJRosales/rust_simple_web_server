/// This file contains the routes to all of your components. To add a new components run the command add, import the component with "use crate::{component_name}", and add the get_routes method to the vector in this file.

use crate::app;
use crate::trial;

pub fn get_routes() -> Vec<(String, String, Vec<(String, String, String)>)> {
    return vec![
        // New routes go here
        app::routes::get_routes(),
        trial::routes::get_routes(),
    ];
}