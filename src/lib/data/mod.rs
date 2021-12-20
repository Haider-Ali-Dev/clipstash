pub mod model;
pub mod query;


use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

use sqlx::Sqlite;

#[derive(Debug, thiserror::Error)]
pub enum DataError {
    #[error("Data base error: {0}")]
    Database(#[from] sqlx::Error)
}

pub type AppDatabase = Database<Sqlite>;
pub type DatabasePool = sqlx::sqlite::SqlitePool;
pub type Transacation<'a> = sqlx::Transaction<'a, Sqlite>;
pub type AppDatabaseRow  = sqlx::sqlite::SqliteRow;
pub type AppQueryResult = sqlx::sqlite::SqliteQueryResult;

pub struct Database<D: sqlx::Database>(sqlx::Pool<D>);

impl Database<Sqlite> {
    pub async fn new(connection_str: &str ) -> Self {
        let pool = sqlx::sqlite::SqlitePoolOptions::new()
            .connect(connection_str)
            .await;
            match pool {
                Ok(pool) => Self(pool),
                Err(e) => {
                    eprintln!("{:?}\n", e);
                    eprintln!("if the database has not yet been created, run \n $ sqlx database setup \n");
                    panic!("Database error");
                }
            }
    }

    pub fn get_pool(&self) -> &DatabasePool {
        &self.0
    }
}


#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DbId(Uuid);

impl DbId {
    /// Create a new database ID.
    pub fn new() -> DbId {
        Uuid::new_v4().into()
    }

    /// Create an empty database ID.
    ///
    /// This database ID is always the same. It can be used to obscure an
    /// actual ID when working with clients.
    pub fn nil() -> DbId {
        Self(Uuid::nil())
    }
}

impl From<DbId> for String {
    fn from(id: DbId) -> Self {
        format!("{}", id.0)
    }
}

/// The default behavior is to create a [`DbId`]
impl Default for DbId {
    fn default() -> Self {
        Self::new()
    }
}


impl FromStr for DbId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DbId(Uuid::parse_str(id)?))
    }
}
