-- Real-world benchmark deterministic seed (BETA-001-A).
--
-- Every row is derived from generate_series indices with modular arithmetic —
-- no random functions — so applying this file always produces the identical
-- dataset. Orders/order_items are intentionally NOT seeded: W2 creates them
-- and reset.sh wipes them by re-applying schema.sql (DROP cascade is not used;
-- schema.sql uses IF NOT EXISTS, so reset.sql drops tables first).

-- 1,000 users: usr_1..usr_1000
INSERT INTO users (id, name, email, role, created_at) (
    SELECT
        'usr_' || i,
        'User ' || i,
        'user' || i || '@benchmark.local',
        CASE WHEN i % 10 = 0 THEN 'admin' ELSE 'user' END,
        TIMESTAMP WITH TIME ZONE '2026-01-01T00:00:00Z' + (i % 86400) * INTERVAL '1 second'
    FROM generate_series(1, 1000) AS i
);

-- 500 products: prod_1..prod_500 across 5 categories (electronics every 5th)
INSERT INTO products (id, title, category, price_cents, stock, created_at) (
    SELECT
        'prod_' || i,
        'Product ' || i,
        (ARRAY['electronics','home','sports','books','toys'])[1 + (i % 5)],
        500 + (i % 200) * 100,
        10 + (i % 90),
        TIMESTAMP WITH TIME ZONE '2026-01-02T00:00:00Z' + (i % 86400) * INTERVAL '1 second'
    FROM generate_series(1, 500) AS i
);

-- 10,000 reviews: deterministic ratings 1..5, 20 per product
INSERT INTO reviews (id, product_id, user_id, rating, comment, created_at) (
    SELECT
        'rev_' || p.i || '_' || r.i,
        'prod_' || p.i,
        'usr_' || (1 + ((p.i * 20 + r.i) % 1000)),
        1 + ((p.i + r.i) % 5),
        CASE WHEN (p.i + r.i) % 3 = 0 THEN 'Great value.' ELSE 'Works as expected.' END,
        TIMESTAMP WITH TIME ZONE '2026-02-01T00:00:00Z' + ((p.i * 20 + r.i) % 86400) * INTERVAL '1 second'
    FROM generate_series(1, 500) AS p(i)
    CROSS JOIN generate_series(1, 20) AS r(i)
);
