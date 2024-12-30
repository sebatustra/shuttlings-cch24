use chrono::Utc;
use sqlx::{Error, PgPool};
use uuid::Uuid;

use super::{Quote, QuoteForCreation, QuoteForUpdate};

#[derive(Clone)]
pub struct QuoteController {
    pub pool: PgPool
}

impl QuoteController {
    pub fn build(pool: PgPool) -> Self {
        Self {
            pool
        }
    }

    pub async fn create_quote(
        &self,
        quote_for_creation: QuoteForCreation
    ) -> Result<Quote, Error> {
        let id = Uuid::new_v4();
        let created_at = Utc::now();
        let version = 1;

        let quote = sqlx::query_as::<_, Quote>(
            "INSERT INTO quotes (id, author, quote, created_at, version) 
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *;"
        )
        .bind(&id)
        .bind(&quote_for_creation.author)
        .bind(&quote_for_creation.quote)
        .bind(&created_at)
        .bind(&version)
        .fetch_one(&self.pool)
        .await?;

        Ok(quote)
    }

    pub async fn get_quote(
        &self,
        id: &Uuid
    ) -> Result<Option<Quote>, Error> {
        let quote =  sqlx::query_as::<_, Quote>("SELECT * FROM quotes WHERE id = $1;")
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(quote)
    }

    pub async fn delete_quote(
        &self,
        id: &Uuid
    ) -> Result<Option<Quote>, Error> {
        let quote = sqlx::query_as::<_, Quote>("DELETE FROM quotes WHERE id = $1 RETURNING *;")
            .bind(&id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(quote)
    }

    pub async fn update_quote(
        &self,
        id: &Uuid,
        quote_for_update: &QuoteForUpdate
    ) -> Result<Option<Quote>, Error> {

        let current_version = sqlx::query_scalar::<_, i32>(
            "SELECT version FROM quotes WHERE id = $1;"
        )
        .bind(&id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(version) = current_version {
            match sqlx::query_as::<_, Quote>(
                "UPDATE quotes 
                SET author = $1, quote = $2, version = $3
                WHERE id = $4
                RETURNING *;"
            )
            .bind(&quote_for_update.author)
            .bind(&quote_for_update.quote)
            .bind(version + 1)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
            {
                Ok(option) => Ok(option),
                Err(e) => Err(e)
            }
        } else {
            Ok(None)
        }



        
    }

    pub async fn clean_db(&self) -> Result<(), Error> {
        match sqlx::query("DELETE FROM quotes;")
            .execute(&self.pool)
            .await 
        {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }
}