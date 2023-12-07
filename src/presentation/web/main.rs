use actix_web::{HttpServer, App, web::Data};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use dotenv::dotenv;

mod routes;
use routes::config as routes;

pub struct AppState {
    db: Pool<Postgres>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let connecrion_string = std::env::var("DATABASE_URL").expect("Строка подлючения должна быть установлена!");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connecrion_string)
        .await.expect("Ошибка пула подключений!");
    
    sqlx::migrate!("./migrations").run(&pool).await.expect("Произошла ошибка миграции!");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState{db: pool.clone()}))
            .configure(routes)
    }).bind(("127.0.0.1", 8080))?.run().await
}
