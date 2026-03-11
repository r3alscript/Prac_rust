<<<<<<< HEAD
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name VARCHAR(255) NOT NULL,
    balance NUMERIC(12,2) NOT NULL DEFAULT 0,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS lots (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    start_price NUMERIC(12,2) NOT NULL,
    min_increment NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    start_at_utc TIMESTAMP NOT NULL,
    end_at_utc TIMESTAMP NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS lot_images (
                                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    url TEXT NOT NULL
    );

CREATE TABLE IF NOT EXISTS bids (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    bidder_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    placed_at_utc TIMESTAMP NOT NULL DEFAULT NOW(),
    is_winning BOOLEAN
    );

CREATE TABLE IF NOT EXISTS payments (
                                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    payer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS notifications (
                                             id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    channel VARCHAR(30) NOT NULL,
    payload_json TEXT NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS auction_results (
                                               id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL UNIQUE REFERENCES lots(id) ON DELETE CASCADE,
    winner_bid_id UUID REFERENCES bids(id) ON DELETE SET NULL,
    winner_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    final_price NUMERIC(12,2) NOT NULL,
    ended_at_utc TIMESTAMP NOT NULL,
    result_status VARCHAR(30) NOT NULL
    );

CREATE TABLE IF NOT EXISTS outbox_events (
                                             id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    payload_json TEXT NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW(),
    published_at_utc TIMESTAMP
=======
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
                                     id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    name VARCHAR(255) NOT NULL,
    balance NUMERIC(12,2) NOT NULL DEFAULT 0,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS lots (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    start_price NUMERIC(12,2) NOT NULL,
    min_increment NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    start_at_utc TIMESTAMP NOT NULL,
    end_at_utc TIMESTAMP NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS lot_images (
                                          id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    url TEXT NOT NULL
    );

CREATE TABLE IF NOT EXISTS bids (
                                    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    bidder_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    placed_at_utc TIMESTAMP NOT NULL DEFAULT NOW(),
    is_winning BOOLEAN
    );

CREATE TABLE IF NOT EXISTS payments (
                                        id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL REFERENCES lots(id) ON DELETE CASCADE,
    payer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    amount NUMERIC(12,2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS notifications (
                                             id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    type VARCHAR(50) NOT NULL,
    channel VARCHAR(30) NOT NULL,
    payload_json TEXT NOT NULL,
    status VARCHAR(30) NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW()
    );

CREATE TABLE IF NOT EXISTS auction_results (
                                               id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    lot_id UUID NOT NULL UNIQUE REFERENCES lots(id) ON DELETE CASCADE,
    winner_bid_id UUID REFERENCES bids(id) ON DELETE SET NULL,
    winner_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    final_price NUMERIC(12,2) NOT NULL,
    ended_at_utc TIMESTAMP NOT NULL,
    result_status VARCHAR(30) NOT NULL
    );

CREATE TABLE IF NOT EXISTS outbox_events (
                                             id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    event_type VARCHAR(100) NOT NULL,
    aggregate_id UUID NOT NULL,
    payload_json TEXT NOT NULL,
    created_at_utc TIMESTAMP NOT NULL DEFAULT NOW(),
    published_at_utc TIMESTAMP
>>>>>>> f63628c3a44df1d65bd9e805f0eef628cd04195e
    );