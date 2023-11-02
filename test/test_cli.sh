#!/bin/bash

set -e

TEMP_DIR=$(mktemp -d)
trap "rm -rf ${TEMP_DIR}" EXIT
OUT_FILE="${TEMP_DIR}/out"
ERR_FILE="${TEMP_DIR}/err"

export IPC_HANDLE_PATH="${TEMP_DIR}/ipc.sock"
# TODO: リクエストが実行される前にテストが落ちるとサーバーが稼働したままになる.後処理を検討.
node test/srv.mjs &

cargo build -q

echo "START SRV"
# ワークフローの RUN で実行した場合、端末からの入力にならないので "" を渡すことで対応.
echo -n "" | ./target/debug/workspace run --memory-initial 100 test.wasm \
    1> "${OUT_FILE}" \
    2> "${ERR_FILE}"
echo "END SRV"


cat "${OUT_FILE}"
echo ""
cat "${ERR_FILE}"
echo ""

diff <(echo -n '/run') "${OUT_FILE}"
diff <(echo -n '["--memory_initial","100","--memory_maximum","0","--memory_shared","true","--","test.wasm"]') "${ERR_FILE}"


# TODO: リクエストが実行される前にテストが落ちるとサーバーが稼働したままになる.後処理を検討.
node test/pipe.mjs &

echo "START PIPE"
HASH_LEFT="$(sha256sum < ./target/debug/workspace )"
HASH_RIGHT="$(./target/debug/workspace run test.wasm < ./target/debug/workspace | sha256sum)"
echo "END PIPE"

test "${HASH_LEFT}" = "${HASH_RIGHT}"


echo "PASS"