-- Create payments table
CREATE TABLE IF NOT EXISTS payments  (
    order_uid TEXT NOT NULL PRIMARY KEY,
    transaction TEXT NOT NULL,
    request_id TEXT,
    currency TEXT NOT NULL,
    provider TEXT NOT NULL,
    amount BIGINT NOT NULL,
    payment_dt BIGINT NOT NULL,
    bank TEXT NOT NULL,
    delivery_cost BIGINT NOT NULL,
    goods_total BIGINT NOT NULL,
    custom_fee BIGINT NOT NULL,
    FOREIGN KEY (order_uid) REFERENCES orders(order_uid) 
        ON DELETE CASCADE 
        ON UPDATE RESTRICT
);