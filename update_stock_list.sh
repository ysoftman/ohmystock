#!/bin/bash
# KRX 상장법인목록 다운로드 후 UTF-8로 변환하여 data/stock_list.html 에 저장
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"
OUTPUT="${DATA_DIR}/stock_list.html"
TMP_FILE=$(mktemp)

echo "downloading stock list from KRX..."
curl -sL -d "" "http://kind.krx.co.kr/corpgeneral/corpList.do?method=download" -o "${TMP_FILE}"

echo "converting EUC-KR to UTF-8..."
iconv -f EUC-KR -t UTF-8 "${TMP_FILE}" | sed 's/charset=euc-kr/charset=utf-8/g' >"${OUTPUT}"

rm -f "${TMP_FILE}"

echo "formatting with prettier..."
bunx prettier --write "${OUTPUT}"

echo "saved to ${OUTPUT}"
