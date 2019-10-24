// See https://rocket.rs/v0.4/guide/testing/#local-dispatching
#[cfg(test)]
mod test {
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use rustlang_rocket_mongodb::rocket;

    #[test]
    fn get_cats() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let response = client.get("/cats").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }

    #[test]
    fn get_cat() {
        // Well get and post tests are identical ...
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/cats")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/cats/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/cats").dispatch();
    }

    #[test]
    fn post_cat() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/cats")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.get(format!("/cats/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/cats").dispatch();
    }

    #[test]
    fn update_cat() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/cats")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let response = client
            .put(format!("/cats/{}", id[3]))
            .header(ContentType::JSON)
            .body(r#"{ "name": "chichi" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let mut response = client.get(format!("/cats/{}", id[3])).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains("chichi"));
        client.delete("/cats").dispatch();
    }

    #[test]
    fn delete_cat() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/cats")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);

        let id = response.body_string().unwrap();
        let id: Vec<&str> = id.split("\"").collect();
        let mut response = client.delete(format!("/cats/{}", id[3])).dispatch();
        assert!(response.body().is_some());
        assert!(response.body_string().unwrap().contains(&id[3]));
        client.delete("/cats").dispatch();
    }

    #[test]
    fn delete_all() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        client.delete("/cats").dispatch();
        let response = client
            .post("/cats")
            .header(ContentType::JSON)
            .body(r#"{ "name": "chacha" }"#)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let response = client.delete("/cats").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
