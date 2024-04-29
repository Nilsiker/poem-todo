mod api;

use api::todos::TodosApi;
use poem::Result;
use poem::{listener::TcpListener, EndpointExt, Route, Server};
use poem_openapi::OpenApiService;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), color_eyre::eyre::Error> {
    color_eyre::install()?;

    let database_url = std::env::var("DATABASE_URL")?;
    let max_connections = std::env::var("DATABASE_MAX_CONNECTIONS")?.parse()?;

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&database_url)
        .await?;

    let api_service = OpenApiService::new(TodosApi, "Todos", "1.0.0");
    let ui = api_service.openapi_explorer();

    let app = Route::new()
        .nest("/", api_service)
        .nest("/ui", ui)
        .data(pool);
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;

    Ok(())
}
