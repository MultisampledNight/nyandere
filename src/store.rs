//! Core of the application.
//!
//! It translates more abstract instructions like "purchase this product"
//! into actual instructions handled by the database.

use sea_orm::{Database, DatabaseConnection, DatabaseTransaction, TransactionTrait};
use thiserror::Error;

use crate::{config::Config, model::Purchase};

/// See [the module docs](crate::handler).
pub struct Store {
    db: DatabaseConnection,
}

impl Store {
    pub async fn new(cfg: Config) -> Result<Self, CreationError> {
        let db = Database::connect(cfg.database).await?;

        Ok(Self { db })
    }

    pub async fn session(&self) -> Result<Session, SessionError> {
        let transaction = self.db.begin().await.map_err(SessionError::Start)?;
        Ok(Session { transaction })
    }
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum CreationError {
    #[error("could not connect to database: {0}")]
    Database(#[from] sea_orm::DbErr),
}

#[derive(Debug, PartialEq, Eq, Error)]
pub enum SessionError {
    #[error("could not start transaction: {0}")]
    Start(sea_orm::DbErr),
    #[error("could not finish transaction: {0}")]
    Finish(sea_orm::DbErr),
    #[error("could not abort transaction: {0}")]
    Abort(sea_orm::DbErr),
}

/// One visit to the outside world, purchasing several products.
pub struct Session {
    transaction: DatabaseTransaction,
}

impl Session {
    pub async fn purchase(&self, purchase: Purchase) {
        todo!();
    }

    /// Wrap up this session, throwing everything that's been done into the database.
    pub async fn finish(self) -> Result<(), SessionError> {
        self.transaction
            .commit()
            .await
            .map_err(SessionError::Finish)
    }

    /// Abort this session, causing the changes to be lost.
    pub async fn abort(self) -> Result<(), SessionError> {
        self.transaction
            .rollback()
            .await
            .map_err(SessionError::Abort)
    }
}
