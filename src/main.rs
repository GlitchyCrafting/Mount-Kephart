#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;

mod routes;
mod database;
mod tests;

#[launch]
fn rocket() -> _ {
    rocket::build()
        // Bind all endpoints to root directory
        .mount("/", routes![
               routes::endpoints::index,
               routes::endpoints::lesson,
               routes::cookies::write,
               routes::cookies::read])
        // Bind static files to their directories
        .mount("/styles", FileServer::from(relative!("static/styles")))
        .mount("/images", FileServer::from(relative!("static/images")))
        .mount("/ace", FileServer::from(relative!("static/ace")))
        // Bind error handlers to the root directory
        .register("/", catchers![
                  routes::error_handlers::not_found,
                  routes::error_handlers::internal_server_error])
        .attach(Template::fairing())
}
