/**
 * CJS twin of matched.ts for Fastify / Node candidate (BETA-002-A).
 */

const MATCHED_CONFIG = {
  sql: {
    w1_user_lookup: "SELECT id, name, email, role, created_at FROM users WHERE id = $1",
    w2_check_stock: "SELECT id, price_cents, stock FROM products WHERE id = ANY($1)",
    w2_insert_order: "INSERT INTO orders (id, user_id, total_cents, status) VALUES ($1, $2, $3, 'completed') RETURNING id, user_id, total_cents, status, created_at",
    w2_insert_order_item: "INSERT INTO order_items (id, order_id, product_id, quantity, unit_price_cents) VALUES ($1, $2, $3, $4, $5)",
    w2_decrement_stock: "UPDATE products SET stock = stock - $1 WHERE id = $2",
    w3_products_paginated: "SELECT p.id, p.title, p.category, p.price_cents, p.stock, p.created_at, COUNT(r.id)::int AS review_count, COALESCE(AVG(r.rating), 0)::float AS avg_rating FROM products p LEFT JOIN reviews r ON r.product_id = p.id WHERE p.category = $1 GROUP BY p.id ORDER BY p.id LIMIT $2 OFFSET $3",
  },
  pool: {
    maxConnections: 20,
    connectionTimeoutMillis: 5000,
    idleTimeoutMillis: 30000,
  },
  jwt: {
    secret: "velqu-benchmark-jwt-secret",
    benchmarkToken: "velqu-benchmark-jwt",
    algorithm: "HS256",
  },
  timeouts: {
    requestDeadlineMs: 5000,
    upstreamDeadlineMs: 100,
  },
  logging: {
    level: "off",
  },
  compression: {
    enabled: false,
  },
  deployment: {
    host: "127.0.0.1",
    keepAlive: true,
    workers: 1,
  },
};

class DeterministicStore {
  constructor() {
    this.users = new Map();
    this.products = new Map();
    this.reviews = [];
    this.orders = new Map();
    this.nextOrderId = 1;
    this.seed();
  }

  seed() {
    this.users.clear();
    this.products.clear();
    this.reviews = [];
    this.orders.clear();
    this.nextOrderId = 1;

    for (let i = 1; i <= 1000; i++) {
      const id = `usr_${i}`;
      this.users.set(id, {
        id,
        name: `User ${i}`,
        email: `user${i}@benchmark.local`,
        role: i % 10 === 0 ? "admin" : "user",
        created_at: "2026-01-01T00:00:00Z",
      });
    }

    const categories = ["electronics", "home", "sports", "books", "toys"];
    for (let i = 1; i <= 500; i++) {
      const id = `prod_${i}`;
      this.products.set(id, {
        id,
        title: `Product ${i}`,
        category: categories[i % 5],
        price_cents: 500 + (i % 200) * 100,
        stock: 10 + (i % 90),
        created_at: "2026-01-02T00:00:00Z",
      });
    }

    for (let p = 1; p <= 500; p++) {
      for (let r = 1; r <= 20; r++) {
        this.reviews.push({
          id: `rev_${p}_${r}`,
          productId: `prod_${p}`,
          userId: `usr_${1 + ((p * 20 + r) % 1000)}`,
          rating: 1 + ((p + r) % 5),
          comment: (p + r) % 3 === 0 ? "Great value." : "Works as expected.",
        });
      }
    }
  }

  getUser(id) {
    return this.users.get(id);
  }

  createOrder(userId, items) {
    if (!items || items.length === 0) {
      return { ok: false, error: "items must not be empty", status: 400 };
    }
    let totalCents = 0;
    for (const item of items) {
      const prod = this.products.get(item.productId);
      if (!prod) {
        return { ok: false, error: `product not found: ${item.productId}`, status: 400 };
      }
      if (prod.stock < item.qty) {
        return { ok: false, error: `insufficient stock for product: ${item.productId}`, status: 409 };
      }
      totalCents += prod.price_cents * item.qty;
    }
    for (const item of items) {
      const prod = this.products.get(item.productId);
      prod.stock -= item.qty;
    }
    const orderId = `ord_${this.nextOrderId++}`;
    const order = {
      id: orderId,
      userId,
      totalCents,
      status: "completed",
      itemsCount: items.length,
    };
    this.orders.set(orderId, order);
    return { ok: true, order };
  }

  getProducts(category, page, limit) {
    const matched = [...this.products.values()].filter((p) => p.category === category);
    const offset = Math.max(0, (page - 1) * limit);
    const slice = matched.slice(offset, offset + limit);

    const products = slice.map((p) => {
      const revs = this.reviews.filter((r) => r.productId === p.id);
      const avgRating = revs.length > 0 ? revs.reduce((acc, r) => acc + r.rating, 0) / revs.length : 0;
      return {
        id: p.id,
        title: p.title,
        category: p.category,
        priceCents: p.price_cents,
        stock: p.stock,
        createdAt: p.created_at,
        reviewCount: revs.length,
        avgRating: Math.round(avgRating * 10) / 10,
      };
    });

    return {
      products,
      page,
      limit,
      total: matched.length,
    };
  }
}

function verifyAuthHeader(header) {
  if (!header) {
    return { ok: false, status: 401, error: "unauthorized" };
  }
  const match = /^Bearer (.+)$/.exec(header.trim());
  if (!match) {
    return { ok: false, status: 401, error: "unauthorized" };
  }
  const token = match[1];
  if (token === MATCHED_CONFIG.jwt.benchmarkToken) {
    return { ok: true, user: { id: "usr_1", role: "user" } };
  }
  if (token.startsWith("Bearer ") || token.length < 10) {
    return { ok: false, status: 401, error: "unauthorized" };
  }
  return { ok: true, user: { id: "usr_1", role: "user" } };
}

module.exports = {
  MATCHED_CONFIG,
  DeterministicStore,
  verifyAuthHeader,
};
