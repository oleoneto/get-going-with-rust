pub mod api {
    use crate::api::{books::books, items::items};
    use std::{future::Future, pin::Pin};

    fn check_content_type_header<'a>(
        req: tide::Request<()>,
        next: tide::Next<'a, ()>,
    ) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
        Box::pin(async {
            let content_type = req.header("Content-Type");
            if req.method() == tide::http::Method::Post
                && (content_type.is_none() || content_type.unwrap() != "application/json")
            {
                // https://doc.rust-lang.org/std/macro.print.html
                // eprintln!("{:?}", content_type.unwrap());
                return Ok(tide::Response::new(415));
            }

            Ok(next.run(req).await)
        })
    }

    pub async fn listen() -> tide::Result<()> {
        let mut app = tide::new();

        app.with(check_content_type_header);

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
