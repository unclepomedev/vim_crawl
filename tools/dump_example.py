import argparse
import sys
from pathlib import Path


def aggregate_rs_files(source_dir, output_file, header_title):
    source_dir_path = Path(source_dir)

    with open(output_file, "w", encoding="utf-8") as outfile:
        outfile.write(f"# {header_title} Examples Reference\n\n")
        outfile.write(
            f"This file contains the official example code for {header_title}. Please strictly follow this as an API reference during implementation.\n\n"
        )

        for rs_file in sorted(source_dir_path.rglob("*.rs")):
            rel_path = rs_file.relative_to(source_dir_path.parent)

            outfile.write(f"## File: {rel_path}\n")
            outfile.write("```rust\n")

            try:
                with open(rs_file, "r", encoding="utf-8") as infile:
                    outfile.write(infile.read())
            except OSError as e:
                outfile.write(f"// Error reading file: {e}\n")

            outfile.write("\n```\n\n")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Generate markdown from Rust examples."
    )
    parser.add_argument("source_dir", help="Path to the examples directory")
    parser.add_argument("output_file", help="Path to the output markdown file")
    parser.add_argument("header_title", help="Title to use in the markdown header")

    args = parser.parse_args()

    source_path = Path(args.source_dir)
    if source_path.is_dir():
        aggregate_rs_files(source_path, args.output_file, args.header_title)
        print(f"success: {args.output_file} generated")
    else:
        print(f"error: directory not found: {args.source_dir}", file=sys.stderr)
        raise SystemExit(1)
