-- Create orders table
CREATE TABLE IF NOT EXISTS orders (
    order_uid TEXT NOT NULL PRIMARY KEY,
    track_number TEXT NOT NULL,
    entry TEXT NOT NULL,
    locale TEXT NOT NULL,
    internal_signature TEXT,
    customer_id TEXT NOT NULL,
    delivery_service TEXT NOT NULL,
    shardkey TEXT NOT NULL,
    sm_id BIGINT NOT NULL,
    date_created TIMESTAMPTZ NOT NULL,
    oof_shard TEXT NOT NULL
);
