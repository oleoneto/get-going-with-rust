pub mod items {
    #[derive(serde::Deserialize, serde::Serialize)]
    struct Item {
        key: String,
        value: String,
    }

    pub struct Repository {
        items: std::collections::HashMap<String, Item>,
    }

    impl Repository {
        fn new() -> Self {
            Self {
                items: std::collections::HashMap::new(),
            }
        }
    }

    // TIP: This state needs to be passed down to all nested routes.
    type State = std::sync::Arc<async_std::sync::RwLock<Repository>>;

    async fn create(mut req: tide::Request<State>) -> tide::Result {
        let item: Item = req.body_json().await?;
        let state = req.state();
        let mut repo = state.write().await;

        repo.items.insert(item.key.clone(), item);

        Ok(tide::Response::new(201))
    }

    async fn list(req: tide::Request<State>) -> tide::Result {
        let state = req.state();
        let repo = &state.read().await;
        let res = tide::Response::builder(200)
            .body(tide::Body::from_json(&repo.items)?)
            .build();

        Ok(res)
    }

    pub fn routes() -> tide::Server<State> {
        let mut repo: Repository = Repository::new();
        repo.items.insert(
            "author".to_string().clone(),
            Item {
                key: "Author".to_string(),
                value: "Leo".to_string(),
            },
        );

        /*
            TIP: Stateful types need to conform to the `Clone+Send+Sync` traits.
            Wrapping `Repository::new()` with `Arc` (automatic reference count) gives us such functionality.
        */
        let persistence = std::sync::Arc::new(async_std::sync::RwLock::new(repo));

        let mut r = tide::with_state(persistence);
        r.at("").post(create);
        r.at("").get(list);

        r
    }
}
