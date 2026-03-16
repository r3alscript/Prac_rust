use crate::domain::lot::Lot;
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct LotRepository;

impl LotRepository {
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Lot>, sqlx::Error> {
        sqlx::query_as::<_, Lot>(
            r#"
            SELECT
                id,
                seller_id,
                title,
                description,
                start_price::float8 as start_price,
                min_increment::float8 as min_increment,
                currency,
                start_at_utc,
                end_at_utc,
                status,
                created_at_utc
            FROM lots
            ORDER BY created_at_utc DESC
            "#
        )
            .fetch_all(pool)
            .await
    }

    pub async fn get_by_id(pool: &PgPool, lot_id: Uuid) -> Result<Option<Lot>, sqlx::Error> {
        sqlx::query_as::<_, Lot>(
            r#"
            SELECT
                id,
                seller_id,
                title,
                description,
                start_price::float8 as start_price,
                min_increment::float8 as min_increment,
                currency,
                start_at_utc,
                end_at_utc,
                status,
                created_at_utc
            FROM lots
            WHERE id = $1
            "#
        )
            .bind(lot_id)
            .fetch_optional(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        seller_id: Uuid,
        title: String,
        description: String,
        start_price: f64,
        auction_end: DateTime<Utc>,
        image_url: Option<String>,
    ) -> Result<Lot, sqlx::Error> {
        let lot_id = Uuid::new_v4();
        let mut tx = pool.begin().await?;

        let auction_end_naive: NaiveDateTime = auction_end.naive_utc();

        let lot = sqlx::query_as::<_, Lot>(
            r#"
            INSERT INTO lots (
                id,
                seller_id,
                title,
                description,
                start_price,
                min_increment,
                currency,
                start_at_utc,
                end_at_utc,
                status,
                created_at_utc
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, NOW(), $8, $9, NOW())
            RETURNING
                id,
                seller_id,
                title,
                description,
                start_price::float8 as start_price,
                min_increment::float8 as min_increment,
                currency,
                start_at_utc,
                end_at_utc,
                status,
                created_at_utc
            "#
        )
            .bind(lot_id)
            .bind(seller_id)
            .bind(title)
            .bind(description)
            .bind(start_price)
            .bind(1.0_f64)
            .bind("UAH")
            .bind(auction_end_naive)
            .bind("ACTIVE")
            .fetch_one(&mut *tx)
            .await?;

        if let Some(url) = image_url {
            sqlx::query(
                r#"
                INSERT INTO lot_images (id, lot_id, url)
                VALUES ($1, $2, $3)
                "#
            )
                .bind(Uuid::new_v4())
                .bind(lot_id)
                .bind(url)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(lot)
    }

    pub async fn count_bids(pool: &PgPool, lot_id: Uuid) -> Result<i64, sqlx::Error> {
        sqlx::query_scalar::<_, i64>(
            r#"
            SELECT COUNT(*)
            FROM bids
            WHERE lot_id = $1
            "#
        )
            .bind(lot_id)
            .fetch_one(pool)
            .await
    }

    pub async fn max_bid(pool: &PgPool, lot_id: Uuid) -> Result<Option<f64>, sqlx::Error> {
        sqlx::query_scalar::<_, Option<f64>>(
            r#"
            SELECT MAX(amount)::float8
            FROM bids
            WHERE lot_id = $1
            "#
        )
            .bind(lot_id)
            .fetch_one(pool)
            .await
    }

    pub async fn current_price(pool: &PgPool, lot_id: Uuid) -> Result<f64, sqlx::Error> {
        sqlx::query_scalar::<_, f64>(
            r#"
            SELECT COALESCE(MAX(b.amount)::float8, l.start_price::float8)
            FROM lots l
            LEFT JOIN bids b ON b.lot_id = l.id
            WHERE l.id = $1
            GROUP BY l.start_price
            "#
        )
            .bind(lot_id)
            .fetch_one(pool)
            .await
    }

    pub async fn first_image_url(
        pool: &PgPool,
        lot_id: Uuid,
    ) -> Result<Option<String>, sqlx::Error> {
        sqlx::query_scalar::<_, Option<String>>(
            r#"
            SELECT url
            FROM lot_images
            WHERE lot_id = $1
            ORDER BY id
            LIMIT 1
            "#
        )
            .bind(lot_id)
            .fetch_optional(pool)
            .await
            .map(|opt| opt.flatten())
    }

    pub async fn place_bid(
        pool: &PgPool,
        lot_id: Uuid,
        bidder_id: Uuid,
        amount: f64,
    ) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let lot: Lot = sqlx::query_as::<_, Lot>(
            r#"
            SELECT
                id,
                seller_id,
                title,
                description,
                start_price::float8 as start_price,
                min_increment::float8 as min_increment,
                currency,
                start_at_utc,
                end_at_utc,
                status,
                created_at_utc
            FROM lots
            WHERE id = $1
            FOR UPDATE
            "#
        )
            .bind(lot_id)
            .fetch_one(&mut *tx)
            .await?;

        let current_price: f64 = sqlx::query_scalar::<_, f64>(
            r#"
            SELECT COALESCE(MAX(amount)::float8, $2::float8)
            FROM bids
            WHERE lot_id = $1
            "#
        )
            .bind(lot_id)
            .bind(lot.start_price)
            .fetch_one(&mut *tx)
            .await?;

        if amount <= current_price {
            return Err(sqlx::Error::Protocol(
                "Bid must be greater than current price".into(),
            ));
        }

        if amount < current_price + lot.min_increment {
            return Err(sqlx::Error::Protocol(
                "Bid must be at least current price + min increment".into(),
            ));
        }

        if lot.seller_id == bidder_id {
            return Err(sqlx::Error::Protocol(
                "Seller cannot bid on own lot".into(),
            ));
        }

        sqlx::query(
            r#"
            INSERT INTO bids (
                id,
                lot_id,
                bidder_id,
                amount,
                currency,
                placed_at_utc,
                is_winning
            )
            VALUES ($1, $2, $3, $4, $5, NOW(), TRUE)
            "#
        )
            .bind(Uuid::new_v4())
            .bind(lot_id)
            .bind(bidder_id)
            .bind(amount)
            .bind(&lot.currency)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }
}