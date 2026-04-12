#!/usr/bin/env bash

# ---------------------------------------------------------------------------
# Paths & constants
# ---------------------------------------------------------------------------

DIR="$(cd "$(dirname "$0")" && pwd)"

HAND_RANKS_PATH="$DIR/resources/HandRanks.dat"
HAND_RANKS_URL="https://raw.githubusercontent.com/christophschmalhofer/poker/master/XPokerEval/XPokerEval.TwoPlusTwo/HandRanks.dat"

BIN_DIR="$DIR/src/bin"

# ---------------------------------------------------------------------------
# Helper functions
# ---------------------------------------------------------------------------

die() { echo "Error: $*" >&2; exit 1; }

download_hand_ranks() {
  echo "HandRanks.dat not found. Downloading..."
  mkdir -p "$(dirname "$HAND_RANKS_PATH")"

  if command -v curl >/dev/null 2>&1; then
    curl -L --fail --retry 3 --retry-delay 2 -C - \
         -o "$HAND_RANKS_PATH" "$HAND_RANKS_URL" \
      || die "Download failed"

  elif command -v wget >/dev/null 2>&1; then
    wget -c --tries=3 --wait=2 \
         -O "$HAND_RANKS_PATH" "$HAND_RANKS_URL" \
      || die "Download failed"

  else
    die "neither curl nor wget is installed"
  fi

  echo "Download complete."
  echo
}

build_project() {
  echo "Building..."
  cargo clippy || exit 1
  cargo build --release --bins --manifest-path "$DIR/Cargo.toml" \
    || die "Build failed"
  echo "Build OK"
  echo
}

run_binary() {
  local src="$1"
  local name timeout_cmd tmp_results bin out code
  name=$(basename "$src" .rs)
  bin="$DIR/target/release/$name"
  timeout_cmd="$TIMEOUT_CMD"
  tmp_results="$TMP_RESULTS"

  if [ -n "$timeout_cmd" ]; then
    out=$("$timeout_cmd" 5s "$bin" 2>&1)
    code=$?
  else
    out=$("$bin" 2>&1)
    code=$?
  fi

  # Prefix each line so interleaved parallel output stays readable
  echo "[$name] ${out//$'\n'/$'\n[$name] '}"

  case "$code" in
    0)   echo "[$name] OK";           echo "PASS:$name"            >> "$tmp_results" ;;
    124) echo "[$name] TIMEOUT";      echo "FAIL:$name:timeout"    >> "$tmp_results" ;;
    *)   echo "[$name] FAILED ($code)"; echo "FAIL:$name:exit $code" >> "$tmp_results" ;;
  esac
}

print_summary() {
  local tmp_results="$1"
  local pass fail
  pass=$(grep -c '^PASS:' "$tmp_results" 2>/dev/null || true)
  fail=$(grep -c '^FAIL:' "$tmp_results" 2>/dev/null || true)

  echo
  echo "----------------------------------------"
  echo "Results: ${pass:-0} passed  ${fail:-0} failed"

  if [ "${fail:-0}" -gt 0 ]; then
    echo
    echo "Failures:"
    grep '^FAIL:' "$tmp_results" | sed 's/^FAIL:/  • /'
    return 1
  fi
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

# 1. Ensure HandRanks.dat is present
[ -f "$HAND_RANKS_PATH" ] || download_hand_ranks

# 2. Collect Rust binaries
RS_FILES=()
while IFS= read -r f; do
  RS_FILES+=("$f")
done < <(find "$BIN_DIR" -maxdepth 1 -name "*.rs" | sort)

[ "${#RS_FILES[@]}" -gt 0 ] || die "No .rs files found in $BIN_DIR"

# 3. Build
build_project

# 4. Run binaries in parallel
TIMEOUT_CMD=$(command -v gtimeout || command -v timeout || true)
TMP_RESULTS="$(mktemp)"
export DIR TIMEOUT_CMD TMP_RESULTS
export -f run_binary

echo "Running problems in parallel"
echo

printf "%s\n" "${RS_FILES[@]}" \
  | xargs -P 0 -I {} bash -c 'run_binary "$@"' _ {}

# 5. Summarise
print_summary "$TMP_RESULTS"
rm -f "$TMP_RESULTS"