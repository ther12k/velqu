# Velqu Application Pack Format — v2 (binary sections)

Status: normative layout frozen by ADR-0025 (M26-001-B). The encoder and
native adapter are built in M26-003; no producer emits this format before
then, and per ADR-0024 unknown versions fail closed. Integrity-vs-
authenticity policy is ADR-0026; debug source sidecars are ADR-0027 —
sources and maps live in an external `<pack>.sources.json` sidecar that
the runtime never reads; this layout has no source or source-map
section by design.

All integers are **little-endian**. All offsets are absolute byte offsets
from the start of the file.

## 1. File anatomy

```text
+-------------------------------+ offset 0
| header (64 bytes)             |
+-------------------------------+ 64
| section directory             |
|   section_count entries       |
|   x 64 bytes each             |
+-------------------------------+ 64 + 64*section_count  (8-aligned by construction)
| section 0 bytes               | 8-aligned
| ...                           |
| section N-1 bytes             |
+-------------------------------+ total_size
```

## 2. Header (64 bytes, fixed)

```text
off  size  field
0    8     magic            = "VELQUQPK" (ASCII)
8    4     format_version   u32 = 2
12   4     header_size      u32 = 64
16   8     total_size       u64 — exact file length in bytes
24   4     section_count    u32 — directory entry count
28   4     reserved         u32 = 0
32   32    reserved         zero bytes (future header extensions live HERE,
                             growing the header via header_size, never by
                             reinterpreting existing fields)
```

Reader rules: `magic` must match exactly; `header_size` must equal 64 for
this mode; `total_size` must equal the actual file length; reserved bytes
must be zero. Any mismatch rejects the pack (fail closed).

## 3. Section directory

`section_count` entries of exactly 64 bytes, starting at offset 64:

```text
off  size  field
0    2     section_id       u16 — from the registered catalog (see §6)
2    2     flags            u16 — bit 0: OPTIONAL; bits 1..15: zero
4    4     reserved         u32 = 0
8    8     offset           u64 — absolute, 8-aligned
16   8     len              u64 — section byte length, len > 0
24   32    content_sha256   SHA-256 of the section's raw bytes
```

Directory rules (all enforced before any section is interpreted):

1. Entries are unique by `section_id`; duplicates reject.
2. `offset >= 64 + 64 * section_count` (sections never overlap header or
   directory).
3. `offset % 8 == 0` and `len > 0`.
4. Ranges `[offset, offset+len)` are disjoint across entries.
5. `offset + len <= total_size`.
6. `content_sha256` must match the bytes at read time. This is
   **integrity only** (ADR-0026): it detects corruption and naive
   tampering, never establishes origin. Authenticity is out-of-band
   deployment policy — detached signatures or build provenance; the
   runtime has no trust anchors and no in-pack signature semantics.

## 4. Optional sections

- Flag bit 0 (`OPTIONAL = 0x0001`) marks a section whose absence the
  native adapter may tolerate. Presence is still fully validated.
- Every section present in the directory is validated identically
  regardless of the flag; the flag only changes how the ADAPTER treats
  absence.
- Which section ids are required vs optional for mode 2 is fixed by the
  section catalog (M26-003-B), not by pack authors.

## 5. Unknown sections and versioning

- An id outside the registered catalog rejects the pack even if flagged
  optional — there is no skip-and-continue path. Extensibility happens by
  bumping `formatVersion` to a new numeric mode through the ADR process,
  never by silently ignoring unrecognized content.
- There are no minor/patch revisions inside mode 2. Any layout change is
  a new mode: one constant flip plus its named native adapter
  (`PackFormatMode::NativeV2` lands with the M26-003 decoder), reviewed
  and tested together, per ADR-0024 migration rule 2.
- Legacy v1 packs remain loadable only through the legacy-v1 JSON
  adapter during the migration window (ADR-0024 §7).

## 6. Section id catalog (reserved names)

Ids are u16. The catalog below is RESERVED now; concrete encodings are
specified by M26-003-B when the encoder lands. Ids 0xF000..0xFFFF are
reserved for experiments and are invalid in production packs.

| id | name (reserved) | required |
|-----|-----------------|----------|
| 0x0001 | strings / name tables | yes |
| 0x0002 | routes | yes |
| 0x0003 | route plans | yes |
| 0x0004 | schema manifest (dense IR) | yes |
| 0x0005 | policies | yes |
| 0x0006 | capability manifest | yes |
| 0x0007 | bundle bytecode (raw quickjs-ng) | no (OPTIONAL; source bundle section supersedes) |
| 0x0008 | contract summary | yes |

The producing compiler writes every required section; a pack missing any
required id rejects even though the directory itself is well-formed.

## 7. Bounds and denial-of-service posture

Every multi-byte quantity consumed by the reader is bounds-checked
against `total_size` and the section's own `len` before use (constraint
11: all queues, bodies, jobs, heap, stack, and deadlines are bounded).
There is no allocation proportional to unvalidated counts: directory
size derives from `section_count` against file length first, then each
section is range-checked. Malformed packs reject with typed errors
(`PackError::Rejected`) before any engine interaction.
