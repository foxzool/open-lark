#!/usr/bin/env python3
"""Check whether cargo llvm-cov coverage meets the configured threshold."""
import json
import subprocess
import sys
from pathlib import Path

MIN_COVERAGE = 65.0


def main() -> int:
    try:
        result = subprocess.run(
            ["cargo", "llvm-cov", "report", "--json", "--summary-only"],
            check=True,
            capture_output=True,
            text=True,
        )
    except subprocess.CalledProcessError as exc:  # pragma: no cover - 仅在外部失败
        sys.stderr.write(exc.stderr)
        return exc.returncode

    try:
        data = json.loads(result.stdout)
        coverage = data["data"][0]["totals"]["regions"]["percent"]
    except (KeyError, IndexError, ValueError, TypeError) as err:
        sys.stderr.write(f"解析覆盖率 JSON 失败: {err}\n")
        return 1

    print(f"Total coverage: {coverage:.2f}%")

    if coverage < MIN_COVERAGE:
        print(
            f"❌ Coverage {coverage:.2f}% is below minimum threshold {MIN_COVERAGE:.2f}%"
        )
        return 1

    print(
        f"✅ Coverage {coverage:.2f}% meets minimum threshold {MIN_COVERAGE:.2f}%"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
