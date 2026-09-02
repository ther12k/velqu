import { defineService } from "@velqu/core";

export interface Item {
  id: string;
  name: string;
  tags: string[];
}

/**
 * Lazy in-memory item store (M4A-009-A feature module fixture). The factory
 * runs on FIRST use at runtime — never during compilation (COMP-002) — and
 * seeds a deterministic corpus so pagination scenarios are reproducible.
 * In-memory only: a learning fixture, not durable persistence.
 */
export const itemsService = defineService("items.service", () => {
  let next = 1;
  const items = new Map<string, Item>();
  for (let i = 1; i <= 12; i++) {
    const id = `itm_${String(i).padStart(3, "0")}`;
    items.set(id, {
      id,
      name: `item-${i}`,
      tags: i % 2 === 0 ? ["even", "seeded"] : ["odd", "seeded"],
    });
    next = i + 1;
  }
  return {
    list(limit: number, cursor: number): { items: Item[]; nextCursor: string | null } {
      const all = [...items.values()];
      const page = all.slice(cursor, cursor + limit);
      const nextCursor = cursor + page.length < all.length ? String(cursor + page.length) : null;
      return { items: page, nextCursor };
    },
    get(id: string): Item | undefined {
      return items.get(id);
    },
    create(name: string, tags: string[]): Item {
      const id = `itm_${String(next++).padStart(3, "0")}`;
      const item: Item = { id, name, tags };
      items.set(id, item);
      return item;
    },
    update(id: string, name: string | undefined, tags: string[] | undefined): Item | undefined {
      const existing = items.get(id);
      if (!existing) return undefined;
      const updated: Item = {
        ...existing,
        ...(name !== undefined ? { name } : {}),
        ...(tags !== undefined ? { tags } : {}),
      };
      items.set(id, updated);
      return updated;
    },
    remove(id: string): Item | undefined {
      const existing = items.get(id);
      if (!existing) return undefined;
      items.delete(id);
      return existing;
    },
  };
});

let instance: ReturnType<typeof itemsService.factory> | null = null;
export function resolve() {
  return (instance ??= itemsService.factory());
}

export default itemsService;
