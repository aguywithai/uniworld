"""
Generate cleaned dictionary files for Thai, Lao, Khmer, and Myanmar
from ICU break iterator dictionary source files.

Reads from: _development/data/dictionaries/*.txt (ICU source)
Writes to:  src/data/dictionaries/*.dict (cleaned, one word per line, UTF-8)

Usage: python _development/scripts/generate_dictionary_data.py
"""

import os
import sys


DICT_DIR = os.path.join(os.path.dirname(__file__), "..", "data", "dictionaries")
OUT_DIR = os.path.join(os.path.dirname(__file__), "..", "..", "src", "data", "dictionaries")

DICTIONARIES = {
    "thai": "thaidict.txt",
    "lao": "laodict.txt",
    "khmer": "khmerdict.txt",
    "myanmar": "burmesedict.txt",
}


def clean_dictionary(input_path: str) -> list:
    """Read an ICU dictionary file, return sorted unique words."""
    words = set()
    with open(input_path, "r", encoding="utf-8-sig") as f:
        for line in f:
            line = line.strip()
            # Skip empty lines and comments
            if not line or line.startswith("#"):
                continue
            words.add(line)
    return sorted(words)


def write_clean_dict(words: list, output_path: str, lang: str) -> None:
    """Write cleaned word list to output file."""
    with open(output_path, "w", encoding="utf-8", newline="\n") as f:
        for word in words:
            f.write(word + "\n")


def main() -> None:
    os.makedirs(OUT_DIR, exist_ok=True)

    total_words = 0
    total_bytes = 0

    for lang, filename in DICTIONARIES.items():
        input_path = os.path.join(DICT_DIR, filename)
        if not os.path.exists(input_path):
            print(f"WARNING: {input_path} not found, skipping {lang}")
            continue

        words = clean_dictionary(input_path)
        output_path = os.path.join(OUT_DIR, f"{lang}.dict")
        write_clean_dict(words, output_path, lang)

        file_size = os.path.getsize(output_path)
        total_words += len(words)
        total_bytes += file_size
        print(f"{lang}: {len(words)} words, {file_size} bytes -> {output_path}")

    print(f"\nTotal: {total_words} words, {total_bytes} bytes ({total_bytes / 1024:.0f} KB)")


if __name__ == "__main__":
    main()
