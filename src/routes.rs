pub mod error_handlers {
    use rocket_dyn_templates::{Template, context};
    use rocket::Request;

   #[catch(404)]
    pub fn not_found(req: &Request) -> Template {
        Template::render("404", context!{ request: req.uri() })
    }

    #[catch(500)]
    pub fn internal_server_error(req: &Request) -> Template {
        Template::render("500", context!{ request: req.uri() })
    }
}

pub mod endpoints {
    use rocket_dyn_templates::{Template, context};

    use crate::database::functions::get_lesson;

    #[get("/")]
    pub fn index() -> Template {
        Template::render("home", context!{})
    }

    #[get("/<id>")]
    pub fn lesson(id: i64) -> Template {
        // Retrieve lesson data from the database
        let lesson_data = get_lesson(id);

        if lesson_data.is_ok() {
             // Unwrap the lesson data
            let lesson_data = lesson_data.unwrap();

            // Pass the data to a template
            Template::render("lesson", context!{
                id,
                name: lesson_data.name,
                content: lesson_data.content,
                code: lesson_data.code,
                answer: lesson_data.answer
            })
       } else {
             // Send error template when lesson is not found
            Template::render("404", context!{ request: id })
       }
   }
}

pub mod cookies {
    use rocket::http::{CookieJar, Cookie};
    use rocket::response::Redirect;
    use rocket::time::Duration;

    #[get("/write/<id>")]
    pub fn write(cookie_manager: &CookieJar<'_>, id: i64) -> Redirect {
        // Create a cookie with the lesson id as the value
        let mut cookie = Cookie::new("lesson", {id + 1}.to_string());

        // Set the cookie's lifetime and verify that it's set
        cookie.set_max_age(Duration::weeks(104));
        assert_eq!(cookie.max_age(), Some(Duration::weeks(104)));

        // Send the cookie to client
        cookie_manager.add(cookie);

        // Redirect to the next lesson
        Redirect::to(uri!(crate::routes::endpoints::lesson(id + 1)))
    }

    #[get("/read")]
    pub fn read(cookie_manager: &CookieJar<'_>) -> Redirect {
        // Get the cookie from the client
        let cookie = cookie_manager.get("lesson")
            .map(|cookie| cookie.value())
            .unwrap()
            .parse::<i64>()
            .unwrap();

        // Redirect to the lesson retrieved from the cookie
        Redirect::to(uri!(crate::routes::endpoints::lesson(cookie)))
    }
}
