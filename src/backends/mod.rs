#[cfg(all(feature = "postgres", feature = "sqlite"))]
compile_error!("features `crate/postgres` and `crate/sqlite` are mutually exclusive");

use crate::Result;

use diesel::{r2d2::ConnectionManager};
use r2d2::{Pool, PooledConnection};

mod user;

#[cfg(feature = "postgres")]
type Connection = diesel::PgConnection;

#[cfg(feature = "sqlite")]
type Connection = diesel::SqliteConnection;

pub struct Backend {
    connection_pool: Pool<ConnectionManager<Connection>>,
}

impl Backend {
    /// Create a new database connection
    pub fn new(database_url: &str) -> Result<Self> {
        let manager = ConnectionManager::<Connection>::new(database_url);

        Ok(Self {
            connection_pool: Pool::builder()
                .min_idle(Some(1))
                .max_size(10)
                .build(manager)?,
        })
    }

    /// Get the database pooled connection
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<Connection>>> {
        Ok(self.connection_pool.get()?)
    }
}
