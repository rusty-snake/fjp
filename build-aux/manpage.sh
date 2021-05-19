#!/bin/bash

set -eo pipefail

INPUT_FILE="$2"
OUTPUT_FILE="$3"

if [[ "$1" == man ]]; then
	if ! command -v rst2man >&-; then
		echo "Could not find rst2man."
		exit 1
	fi
	rst2man "$INPUT_FILE" | gzip --best --keep --force --no-name > "$OUTPUT_FILE"
elif [[ "$1" == html ]]; then
	if ! command -v rst2html5 >&-; then
		echo "Could not find rst2html5."
		exit 1
	fi
	rst2html5 "$INPUT_FILE" "$OUTPUT_FILE"
else
	echo "Unknow output type (supported: man, html)."
	exit 1
fi
