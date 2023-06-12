pub mod books {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Book {
        title: String,
        genre: String,
    }

    #[derive(serde::Deserialize, Default)]
    struct QueryParams {
        pub title: String,
        pub year: String,
    }

    // Extract query parameters
    async fn search(req: tide::Request<()>) -> tide::Result<String> {
        let query: QueryParams = req.query()?;
        /*
            TIP:
            Adding a new property to the struct makes such property required.
            Not passing it in the request will cause `400 - Bad Request` after `req.query()` is run.
        */
        Ok(format!(
            "Hard Questions: \nTitle: {}\nYear: {}",
            query.title, query.year
        ))
    }

    // Path parameters
    async fn detail(req: tide::Request<()>) -> tide::Result<String> {
        let id = req.param("id").unwrap_or("");
        Ok(format!("Details for book {}", id))
    }

    // JSON Request
    async fn create(mut req: tide::Request<()>) -> tide::Result<String> {
        let book: Book = req.body_json().await?;
        /*
            TIP:
            Adding a new property to the struct makes such property required.
            Failing to pass it in the request will cause `422 - Unprocessable entity` after `req.body_json()` is run.
        */
        Ok(format!("Title: {}, Genre: {}", book.title, book.genre))
    }

    // JSON Response
    async fn list(_req: tide::Request<()>) -> tide::Result<tide::Body> {
        let books = vec![
            Book {
                title: "What is so amazing about grace?".to_string(),
                genre: "Religious".to_string(),
            },
            Book {
                title: "Memoir".to_string(),
                genre: "Biography".to_string(),
            },
        ];

        Ok(tide::Body::from_json(&books)?)
    }

    pub fn routes() -> tide::Server<()> {
        let mut r = tide::new();

        r.at("/find").get(search);
        r.at("/:id").get(detail);
        r.at("").post(create);
        r.at("").get(list);

        r
    }
}
