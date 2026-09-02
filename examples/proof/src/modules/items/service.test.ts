/**
 * Items feature-module unit tests (M4A-009-A): pagination, CRUD, and
 * missing-item semantics against the lazy in-memory store.
 */
import { describe, it, expect } from "bun:test";
import { resolve } from "./service";

describe("items service", () => {
  it("seeds a deterministic corpus and paginates with cursors", () => {
    const svc = resolve();
    const page1 = svc.list(5, 0);
    expect(page1.items.length).toBe(5);
    expect(page1.items[0].id).toBe("itm_001");
    expect(page1.nextCursor).toBe("5");

    const page2 = svc.list(5, Number(page1.nextCursor));
    expect(page2.items[0].id).toBe("itm_006");

    const last = svc.list(5, Number(page2.nextCursor));
    expect(last.items.length).toBe(2);
    expect(last.nextCursor).toBeNull();
  });

  it("clamps page size to the store and returns an empty last page safely", () => {
    const svc = resolve();
    const big = svc.list(50, 0);
    expect(big.items.length).toBe(12);
    expect(big.nextCursor).toBeNull();
    const past = svc.list(5, 99);
    expect(past.items.length).toBe(0);
    expect(past.nextCursor).toBeNull();
  });

  it("creates, reads, updates, and deletes items", () => {
    const svc = resolve();
    const created = svc.create("widget", ["new"]);
    expect(created.id).toMatch(/^itm_[0-9]+$/);

    expect(svc.get(created.id)?.name).toBe("widget");

    const renamed = svc.update(created.id, "gadget", undefined);
    expect(renamed?.name).toBe("gadget");
    expect(renamed?.tags).toEqual(["new"]);

    const removed = svc.remove(created.id);
    expect(removed?.id).toBe(created.id);
    expect(svc.get(created.id)).toBeUndefined();
    expect(svc.remove(created.id)).toBeUndefined();
  });
});
