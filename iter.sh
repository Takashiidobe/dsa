#!/usr/bin/env bash


find src/ -type f -name "*.rs" | while IFS= read -r f; do
    doc_path="docs/${f%.rs}.html"
    if [[ ! -e "$doc_path" ]]; then
        echo "Documentation for $f does not exist."
        pycco -p $f  # Uncomment to generate documentation when needed
        continue  # Skip this iteration if the documentation file does not exist
    fi
    if [[ $f -nt $doc_path ]]; then
        echo "File $f is newer than generated $doc_path"
        pycco -p $f  # Uncomment to generate documentation when needed
    fi
done

