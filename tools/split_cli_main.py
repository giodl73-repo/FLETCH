#!/usr/bin/env python3
"""Split fletch-cli main.rs into ROUTE-style modules."""
from __future__ import annotations

import re
from collections import defaultdict
from pathlib import Path

SRC = Path(__file__).resolve().parents[1] / "crates" / "fletch-cli" / "src"
MAIN = SRC / "main.rs"


def brace_end(lines: list[str], start: int) -> int:
    bal = 0
    started = False
    for j in range(start, len(lines)):
        for ch in lines[j]:
            if ch == "{":
                bal += 1
                started = True
            elif ch == "}":
                bal -= 1
                if started and bal == 0:
                    return j
    raise SystemExit(f"unclosed {start + 1}")


def const_end(lines: list[str], i: int) -> int:
    j = i
    br = ba = pa = 0
    seen = False
    while j < len(lines):
        line = lines[j]
        if "=" in line:
            seen = True
        for ch in line:
            if ch == "[":
                br += 1
            elif ch == "]":
                br -= 1
            elif ch == "{":
                ba += 1
            elif ch == "}":
                ba -= 1
            elif ch == "(":
                pa += 1
            elif ch == ")":
                pa -= 1
        if seen and br == ba == pa == 0 and ";" in line:
            return j
        j += 1
    raise SystemExit(f"bad const {i + 1}")


def item_start(lines: list[str], i: int) -> int:
    s = i
    while s > 0:
        p = lines[s - 1].lstrip()
        if (
            p.startswith("///")
            or p.startswith("//!")
            or p.startswith("#[")
            or (p.startswith("//") and "moved" not in p)
        ):
            s -= 1
            continue
        break
    return s


def pubify(chunk: str, kind: str, name: str) -> str:
    for pat, rep in [
        (rf"^(pub\(crate\)\s+|pub\s+)?fn {re.escape(name)}\b", f"pub(crate) fn {name}"),
        (
            rf"^(pub\(crate\)\s+|pub\s+)?struct {re.escape(name)}\b",
            f"pub(crate) struct {name}",
        ),
        (
            rf"^(pub\(crate\)\s+|pub\s+)?enum {re.escape(name)}\b",
            f"pub(crate) enum {name}",
        ),
        (
            rf"^(pub\(crate\)\s+|pub\s+)?const {re.escape(name)}\b",
            f"pub(crate) const {name}",
        ),
    ]:
        n = re.sub(pat, rep, chunk, count=1, flags=re.M)
        if n != chunk:
            chunk = n
            break
    if kind != "struct":
        return chunk
    out: list[str] = []
    inb = False
    bal = 0
    for line in chunk.splitlines(keepends=True):
        if re.search(rf"\bstruct {re.escape(name)}\b", line) and "{" in line:
            inb = True
        if inb:
            bal += line.count("{") - line.count("}")
            m = re.match(
                r"^(\s+)((?:pub(?:\([^)]*\))?\s+)?)([A-Za-z_][\w]*\s*:)",
                line,
            )
            if (
                m
                and not line.lstrip().startswith("//")
                and not line.lstrip().startswith("#[")
            ):
                ind, vis, rest = m.group(1), m.group(2), m.group(3)
                if not vis.strip():
                    line = f"{ind}pub(crate) {rest}{line[m.end() :]}"
                elif vis.strip() == "pub":
                    line = f"{ind}pub(crate) {rest}{line[m.end() :]}"
            if bal <= 0:
                inb = False
        out.append(line)
    return "".join(out)


def domain_fn(name: str) -> str | None:
    if name == "main":
        return None
    if name.startswith(("run_", "cmd_")):
        return "commands"
    if name.startswith("print_"):
        return "print"
    if name.startswith(("parse_", "read_", "write_", "load_", "fetch_", "follow_")):
        return "io"
    if name.startswith(("verify_", "validate_", "check_", "assert_")):
        return "validate"
    return "misc"


def extract_uses(lines: list[str]) -> str:
    buf: list[str] = []
    j = 0
    while j < len(lines):
        if lines[j].startswith("#!"):
            buf.append(lines[j])
            j += 1
            continue
        if lines[j].startswith("use "):
            k = j
            chunk = lines[k]
            while True:
                open_b = chunk.count("{")
                close_b = chunk.count("}")
                if open_b == close_b and (
                    chunk.rstrip().endswith(";") or "};" in chunk
                ):
                    break
                k += 1
                if k >= len(lines):
                    break
                chunk += lines[k]
            buf.extend(lines[j : k + 1])
            j = k + 1
            continue
        if lines[j].strip() == "" and buf:
            j += 1
            continue
        break
    return "".join(buf)


def main() -> None:
    lines = MAIN.read_text(encoding="utf-8", errors="replace").splitlines(keepends=True)
    use_text = extract_uses(lines)

    items: list[tuple[str, str, int, int]] = []
    i = 0
    while i < len(lines):
        raw = lines[i].rstrip("\r\n")
        m = re.match(
            r"^(?:pub(?:\(crate\))?\s+)?(fn|struct|enum|const|static|type)\s+(\w+)",
            raw,
        )
        if m:
            kind, name = m.group(1), m.group(2)
            s = item_start(lines, i)
            if kind == "fn":
                e = brace_end(lines, i)
            elif kind in ("struct", "enum"):
                e = i if ("{" not in raw and ";" in raw) else brace_end(lines, i)
            else:
                e = const_end(lines, i)
            items.append((kind, name, s, e))
            i = e + 1
            continue
        if raw.startswith("impl ") or raw.startswith("impl<"):
            e = brace_end(lines, i)
            s = item_start(lines, i)
            nm = re.search(r"\bfor\s+(\w+)", raw) or re.search(
                r"impl(?:\s*<[^>]+>)?\s+(\w+)", raw
            )
            items.append(("impl", nm.group(1) if nm else "impl", s, e))
            i = e + 1
            continue
        i += 1

    buckets: dict[str, list] = defaultdict(list)
    main_it = None
    for kind, name, s, e in items:
        if kind == "fn" and name == "main":
            main_it = (s, e)
            continue
        if kind in ("struct", "enum") and (
            name in ("Cli", "Commands")
            or name.endswith("Args")
            or name.endswith("Command")
            or "Cli" in name
        ):
            buckets["cli"].append((kind, name, s, e))
            continue
        if kind == "fn":
            d = domain_fn(name) or "misc"
        elif kind in ("struct", "enum", "type", "impl"):
            d = "types"
        elif kind in ("const", "static"):
            d = "constants"
        else:
            d = "misc"
        buckets[d].append((kind, name, s, e))

    header = (
        "//! Split from main.rs (ROUTE-style layout).\n"
        "#![allow(unused_imports, dead_code, unused_variables)]\n"
        "use crate::*;\n"
        f"{use_text}\n"
    )

    support = SRC / "support"
    support.mkdir(exist_ok=True)
    for d, its in buckets.items():
        its = sorted(its, key=lambda x: x[2])
        chunks = []
        for kind, name, s, e in its:
            chunk = "".join(lines[s : e + 1])
            if kind != "impl":
                chunk = pubify(chunk, kind, name)
            chunks.append(chunk.rstrip() + "\n\n")
        path = SRC / f"{d}.rs" if d in ("cli", "types", "constants", "commands") else support / f"{d}.rs"
        path.parent.mkdir(parents=True, exist_ok=True)
        body = header + "".join(chunks)
        path.write_text(body, encoding="utf-8")
        print(f"wrote {path.relative_to(SRC)} n={len(its)} L={body.count(chr(10))+1}")

    smods = sorted(p.stem for p in support.glob("*.rs") if p.name != "mod.rs")
    (support / "mod.rs").write_text(
        "#![allow(unused_imports)]\n"
        + "".join(f"pub(crate) mod {m};\n" for m in smods)
        + "".join(f"pub(crate) use {m}::*;\n" for m in smods),
        encoding="utf-8",
    )

    assert main_it is not None
    ms, me = main_it
    main_body = "".join(lines[ms : me + 1])
    mod_lines = []
    use_lines = []
    for m in ("constants", "cli", "types", "commands", "support"):
        if m == "support" or m in buckets:
            mod_lines.append(f"mod {m};")
            use_lines.append(f"pub(crate) use {m}::*;")

    new = (
        f"{use_text}\n"
        + "\n".join(mod_lines)
        + "\n\n"
        + "\n".join(use_lines)
        + "\n\n"
        + main_body
        + "\n"
    )
    MAIN.write_text(new, encoding="utf-8")
    print("main L", new.count("\n") + 1)
    print("buckets", {k: len(v) for k, v in buckets.items()})


if __name__ == "__main__":
    main()
