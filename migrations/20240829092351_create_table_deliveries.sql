--Create deliveries tabel
CREATE TABLE IF NOT EXISTS deliveries (
    order_uid TEXT NOT NULL PRIMARY KEY ,
    name TEXT NOT NULL,
    phone TEXT NOT NULL,        
    zip TEXT NOT NULL,
    city TEXT NOT NULL,
    address TEXT NOT NULL,
    region TEXT NOT NULL,
    email TEXT NOT NULL,
    FOREIGN KEY (order_uid) REFERENCES orders(order_uid) 
        ON DELETE CASCADE 
        ON UPDATE RESTRICT
);