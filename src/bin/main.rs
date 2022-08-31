#[actix_web::main]
async fn main() -> std::io::Result<()> {
    multi_nodes::init_server().await
}
