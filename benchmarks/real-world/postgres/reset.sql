-- Drop all benchmark tables in FK-safe order so schema.sql can be re-applied.
-- Kept separate from schema.sql so the schema file stays the canonical DDL.
DROP TABLE IF EXISTS order_items;
DROP TABLE IF EXISTS orders;
DROP TABLE IF EXISTS reviews;
DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS users;
