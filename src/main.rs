/*
	TIP: Like in C-based languages, you can import namespaces.
	I won't import them here because I want to be explicit about
	where my dependencies come from:
*/
// use tide::Request;
// use tide::Response;
// std::collections::HashMap;
// std::sync::Arc;
// use async_std::sync::RwLock;

// Return a string
async fn healthz(_req: tide::Request<()>) -> tide::Result<String> {
	Ok("OK".to_string())
}

// Extract query parameters - Example 1
async fn random(req: tide::Request<()>) -> tide::Result<String> {
	let query = req.url()
		.query_pairs()
		.find(|(k, _)| k == "q")
		.map(|(_, v)| v);

	Ok(format!("Question: {}", query.unwrap_or("".into())))
}

// Extract query parameters - Example 2
async fn book_search(req: tide::Request<()>) -> tide::Result<String> {
	#[derive(serde::Deserialize)]
	#[derive(Default)]
	struct QueryParams {
		pub title: String,
		pub year: String,
	}

	let query: QueryParams = req.query()?;
	/*
		TIP:
		Adding a new property to the struct makes such property required.
		Not passing it in the request will cause `400 - Bad Request` after `req.query()` is run.
	*/
	Ok(format!("Hard Questions: \nTitle: {}\nYear: {}", query.title, query.year))
}

// Path parameters
async fn book_detail(req: tide::Request<()>) -> tide::Result<String> {
	let id = req.param("id").unwrap_or("");
	Ok(format!("Details for book {}", id))
}

// JSON Request
async fn book_create(mut req: tide::Request<()>) -> tide::Result<String> {
	#[derive(serde::Deserialize)]
	struct Book {
		title: String,
		genre: String,
	}


	let book: Book = req.body_json().await?;
	/*
		TIP:
		Adding a new property to the struct makes such property required.
		Failing to pass it in the request will cause `422 - Unprocessable entity` after `req.body_json()` is run.
	*/
	Ok(format!("Title: {}, Genre: {}", book.title, book.genre))
}

// JSON Response
async fn book_list(_req: tide::Request<()>) -> tide::Result<tide::Body> {
	#[derive(serde::Serialize)]
	struct Book {
		title: String,
		genre: String,
	}

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

// Returning custom HTTP status code and body
async fn custom_error(_req: tide::Request<()>) -> tide::Result {
	let mut res = tide::Response::new(418);
	res.set_body("Sometimes things do not go to plan... 🤷");

	// TIP: alternatively, you could use the following syntax:
	// let res = tide::Response::new(418).body("Sometimes things do not go to plan... 🤷").build();

	Ok(res)
}

// ======================================================
// ======================================================
// ======================================================

#[async_std::main]
async fn main() -> tide::Result<()> {
	let mut app = tide::new();

	app.at("/").serve_file("./public/index.html")?;

	app.at("/public").serve_dir("./public")?;

	app.at("/").post(|_| async { Ok("Oi!") });

	app.at("/healthz").get(healthz);

	app.at("/random").get(random);

	app.at("/books").nest({
		let mut books = tide::new();

		books.at("/find").get(book_search);
		books.at("/:id").get(book_detail);
		books.at("").post(book_create);
		books.at("").get(book_list);

		books // <- returns
	});

	app.at("/error").get(custom_error);

	// Stateful routes. Persisted data.
	app.at("/items").nest({
		#[derive(serde::Deserialize,serde::Serialize)]
		struct Item {
			key: String,
			value: String,
		}

		struct Repository {
			items: std::collections::HashMap<String, Item>,
		}

		impl Repository {
			fn new() -> Self { Self { items: std::collections::HashMap::new() } }
		}

		/*
			TIP: Stateful types need to conform to the `Clone+Send+Sync` traits
			Wrapping `Repository::new()` with `Arc` (automatic reference count)
			gives us such functionality.
		*/
		let persistence = std::sync::Arc::new(
			async_std::sync::RwLock::new(Repository::new())
		);

		let mut items = tide::with_state(persistence);

		// TIP: This state needs to be passed down to all nested routes.
		type State = std::sync::Arc<async_std::sync::RwLock<Repository>>;

		async fn item_create(mut req: tide::Request<State>) -> tide::Result {
			let item: Item = req.body_json().await?;
			let state = req.state();
			let mut repo = state.write().await;

			repo.items.insert(item.key.clone(), item);
			Ok(tide::Response::new(201))
		}
		items.at("").post(item_create);

		async fn item_list(req: tide::Request<State>) -> tide::Result {
			let state = req.state();
			let repo = &state.read().await;
			let res = tide::Response::builder(200)
				.body(tide::Body::from_json(&repo.items)?)
				.build();

			Ok(res)
		}
		items.at("").get(item_list);

		items // <- returns
	});

	// Logger middleware has no effect on its own.
	// `femme` crate was added to handle logging.
	femme::start();
	app.with(tide::log::LogMiddleware::new());

	app.listen("0.0.0.0:8404").await?;

	Ok(())
}
