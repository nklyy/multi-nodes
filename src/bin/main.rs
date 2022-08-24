#[actix_web::main]
async fn main() -> std::io::Result<()> {
    rpc_api::init_server().await
}
