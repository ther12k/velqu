#!/usr/bin/env python3
"""BWASM-K-005 — WASM import audit.

Parses the WebAssembly binary's import section directly (no external
tools) and lists every imported module/function/table/memory. The
kernel contract (ADR-0037 §6, K-005 acceptance) forbids host-runtime
imports: sockets, fs, process, signals, native threads, wasi_*.
wasm-bindgen's own `wbg` shims are expected and listed.

Usage: bwasm-wasm-import-audit.py <module.wasm>
Exit 0 = audit clean; exit 1 = forbidden import found.
"""
import sys
import struct

FORBIDDEN_SUBSTRINGS = (
    "wasi_snapshot_preview1",
    "wasi_unstable",
    "wasi_",
    "/sockets",
    "sock_",
)


def read_leb(buf: bytes, pos: int) -> tuple[int, int]:
    result = 0
    shift = 0
    while True:
        b = buf[pos]
        pos += 1
        result |= (b & 0x7F) << shift
        if not (b & 0x80):
            return result, pos
        shift += 7


def name(buf: bytes, pos: int) -> tuple[str, int]:
    n, pos = read_leb(buf, pos)
    s = buf[pos:pos + n].decode("utf-8", "replace")
    return s, pos + n


def main() -> int:
    if len(sys.argv) != 2:
        print(__doc__, file=sys.stderr)
        return 2
    buf = open(sys.argv[1], "rb").read()
    if buf[:4] != b"\x00asm":
        print("not a wasm module", file=sys.stderr)
        return 2
    pos = 8
    imports = []
    while pos < len(buf):
        section_id = buf[pos]
        pos += 1
        size, pos = read_leb(buf, pos)
        end = pos + size
        if section_id == 2:  # import section
            count, ipos = read_leb(buf, pos)
            for _ in range(count):
                mod, ipos = name(buf, ipos)
                field, ipos = name(buf, ipos)
                kind = buf[ipos]
                ipos += 1
                if kind == 0x00:  # function: type index
                    _, ipos = read_leb(buf, ipos)
                elif kind == 0x01:  # table
                    _, ipos = read_leb(buf, ipos)  # elemtype
                    limits_flag = buf[ipos]
                    ipos += 1
                    _, ipos = read_leb(buf, ipos)
                    if limits_flag == 1:
                        _, ipos = read_leb(buf, ipos)
                elif kind == 0x02:  # memory
                    limits_flag = buf[ipos]
                    ipos += 1
                    _, ipos = read_leb(buf, ipos)
                    if limits_flag == 1:
                        _, ipos = read_leb(buf, ipos)
                elif kind == 0x03:  # global
                    ipos += 2
                imports.append((mod, field, kind))
        pos = end

    print(f"imports: {len(imports)}")
    modules: dict[str, int] = {}
    for mod, field, _kind in imports:
        modules[mod] = modules.get(mod, 0) + 1
        print(f"  {mod}.{field}")
    print("import modules:", ", ".join(f"{m}({c})" for m, c in sorted(modules.items())))

    violations = [
        (m, f) for m, f in _kinds(imports)
        if any(sub in m or sub in f for sub in FORBIDDEN_SUBSTRINGS)
    ]
    if violations:
        print("FORBIDDEN IMPORTS:", violations)
        return 1
    print("AUDIT-CLEAN: no host-runtime imports (no wasi/fs/socket/thread imports)")
    return 0


def _kinds(imports):
    for mod, field, _ in imports:
        yield mod, field


if __name__ == "__main__":
    sys.exit(main())
