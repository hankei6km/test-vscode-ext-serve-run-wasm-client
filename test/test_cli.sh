#!/bin/bash

set -e

TEMP_DIR=$(mktemp -d)
trap "rm -rf ${TEMP_DIR}" EXIT
OUT_FILE="${TEMP_DIR}/out"
ERR_FILE="${TEMP_DIR}/err"

export IPC_HANDLE_PATH="${TEMP_DIR}/ipc.sock"
node test/srv.mjs &

cargo build -q
echo "START"
./target/debug/workspace run --memory-initial 100 test.wasm \
    1> "${OUT_FILE}" \
    2> "${ERR_FILE}"

cat "${OUT_FILE}"
cat "${ERR_FILE}"

diff <<< '"/run"' "${OUT_FILE}" -
diff <<< '["--memory_initial","100","--memory_maximum","0","--memory_shared","true","--","test.wasm"]' "${ERR_FILE}" -

echo "PASS"