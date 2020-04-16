use crate::api_error::ApiError;
use crate::config::{Config, IConfig};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use lazy_static::lazy_static;
use r2d2;
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let config = Config {};
        let db_url = config.get_config_with_key("DATABASE_URL");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get().map_err(|e| {
        ApiError::new(
            500,
            format!("Failed getting db connection: {}", e),
            "generic".to_string(),
        )
    })
}
