pub mod api {
    use crate::api::books::books;
    use crate::api::items::items;

    pub async fn listen() -> tide::Result<()> {
        let mut app = tide::new();

        app.at("/books").nest(books::routes());

        // Stateful routes. Persisted data.
        app.at("/items").nest(items::routes());

        // Logging with `femme`.
        femme::start();
        app.with(tide::log::LogMiddleware::new());

        app.listen("0.0.0.0:8404").await?;

        Ok(())
    }
}
