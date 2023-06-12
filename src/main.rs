mod api;

#[async_std::main]
async fn main() -> tide::Result<()> {
    return api::api::api::listen().await;
}
