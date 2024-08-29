--Craeate items table
CREATE TABLE IF NOT EXISTS items (
    order_uid TEXT NOT NULL,
    chrt_id BIGINT NOT NULL,
    track_number TEXT NOT NULL,
    price BIGINT NOT NULL,
    rid TEXT NOT NULL,
    name TEXT NOT NULL,
    sale BIGINT NOT NULL,
    size TEXT NOT NULL,
    total_price BIGINT NOT NULL,
    nm_id BIGINT NOT NULL,
    brand TEXT NOT NULL,
    status BIGINT NOT NULL,
    FOREIGN KEY (order_uid) REFERENCES orders(order_uid) 
        ON DELETE CASCADE 
        ON UPDATE RESTRICT
);
