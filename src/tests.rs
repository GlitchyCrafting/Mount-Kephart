#[cfg(test)]
mod test {
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::Status;

    use crate::routes::endpoints;

    #[test]
    fn index_test() {
        // Create client instance
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        // Send a request
        let response = client.get(uri!(endpoints::index)).dispatch();
        // Check that response is status 200 (Ok)
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn lesson_test() {
        // Create client instance
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        // Send a request
        let response = client.get(uri!(endpoints::lesson(1))).dispatch();
        // Check that response is status 200 (Ok)
        assert_eq!(response.status(), Status::Ok);
    }
}
