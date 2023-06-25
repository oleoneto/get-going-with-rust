mod api;

#[async_std::main]
async fn main() -> tide::Result<()> {
    return api::api::api::listen("8404".to_string()).await;
}
