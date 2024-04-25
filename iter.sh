#!/usr/bin/env bash

find src/ -type f \( -name "*.rs" -o -name "*.c" \) | while IFS= read -r f; do
    doc_path="docs/${f%.rs}.html"
    if [[ ! -e "$doc_path" ]]; then
        echo "Documentation for $f does not exist."
        pycco -p $f
        continue
    fi
    if [[ $f -nt $doc_path ]]; then
        echo "File $f is newer than generated $doc_path"
        pycco -p $f
    fi
done

# copy the lib.html file to index.html
cp docs/src/lib.html docs/src/index.html
