#!/usr/bin/env bash
# Build one program (or all) and check the stripped result against the sha256 of the deployed binary.
#
#   ./build.sh router          # one program
#   ./build.sh all             # every program under programs/
#
# Needs: Linux x86_64, curl, tar, bzip2, sha256sum. Toolchains are Anza's published platform-tools
# releases (https://github.com/anza-xyz/platform-tools/releases), downloaded once into .toolchains/.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")" && pwd)
TC=$ROOT/.toolchains
OUT=$ROOT/out

fetch() {  # fetch <version> <host> [member]
  local ver=$1 host=$2 member=${3:-} dir=$TC/$1/$2
  [ -e "$dir/.done${member:+-$(echo "$member" | tr '/' '_')}" ] && return 0
  mkdir -p "$dir"
  local url=https://github.com/anza-xyz/platform-tools/releases/download/$ver/platform-tools-$host.tar.bz2
  echo ">> downloading $url" >&2
  if [ -n "$member" ]; then
    curl -fsSL "$url" | tar -xj -C "$dir" --wildcards "*$member/*"
  else
    curl -fsSL "$url" | tar -xj -C "$dir"
  fi
  touch "$dir/.done${member:+-$(echo "$member" | tr '/' '_')}"
}

build_one() {
  local p=$1 conf=$ROOT/programs/$1/build.conf
  [ -f "$conf" ] || { echo "no such program: $p" >&2; return 1; }
  local PROGRAM_ID DEPLOY_SLOT PT_VER TRIPLE STD PKG LIBTGT EXPECT_SHA256 HOST=""
  # shellcheck disable=SC1090
  . "$conf"
  fetch "$PT_VER" linux-x86_64
  local PT=$TC/$PT_VER/linux-x86_64 sysroot_arg="" rustc=$TC/$PT_VER/linux-x86_64/rust/bin/rustc
  if [ -n "$HOST" ]; then
    # The original build ran on $HOST. cargo takes its host from `rustc -vV`, and the host feeds the
    # metadata hash of every crate, so report it; every other rustc call runs unchanged.
    rustc=$TC/rustc-$PT_VER-$HOST.sh
    printf '#!/bin/sh\nR=%s\nif [ "$1" = "-vV" ]; then "$R" -vV | sed "s/^host: .*/host: %s/"; exit 0; fi\nexec "$R" "$@"\n' \
      "$PT/rust/bin/rustc" "$HOST" > "$rustc"
    chmod +x "$rustc"
  fi
  if [ "$STD" != linux ]; then
    # Link against the standard library shipped in the macOS platform-tools archive of the same release.
    fetch "$PT_VER" "$STD" "rust/lib/rustlib/*-solana-solana"
    local SR=$TC/sysroot-$PT_VER-$STD
    mkdir -p "$SR/lib/rustlib"
    ln -sfn "$PT/rust/lib/rustlib/x86_64-unknown-linux-gnu" "$SR/lib/rustlib/x86_64-unknown-linux-gnu"
    for d in "$TC/$PT_VER/$STD"/rust/lib/rustlib/*-solana-solana; do
      ln -sfn "$d" "$SR/lib/rustlib/$(basename "$d")"
    done
    sysroot_arg=" --sysroot $SR"
  fi
  local rf="-C lto=no -C opt-level=3 -Zremap-cwd-prefix=$sysroot_arg"
  local tvar
  tvar=CARGO_TARGET_$(echo "$TRIPLE" | tr 'a-z-' 'A-Z_')_RUSTFLAGS
  mkdir -p "$OUT"
  (
    cd "$ROOT/programs/$p"
    unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS CARGO_INCREMENTAL RUSTC_WRAPPER
    export PATH=$PT/rust/bin:$PT/llvm/bin:$PATH RUSTC=$rustc CC=$PT/llvm/bin/clang AR=$PT/llvm/bin/llvm-ar
    export "$tvar=$rf" CARGO_TARGET_DIR=$ROOT/target/$p
    "$PT/rust/bin/cargo" build --release --locked --target "$TRIPLE"
    "$PT/llvm/bin/llvm-objcopy" --strip-all "$CARGO_TARGET_DIR/$TRIPLE/release/$LIBTGT.so" "$OUT/$p.so"
  )
  local got
  got=$(sha256sum "$OUT/$p.so" | cut -c1-64)
  if [ "$got" = "$EXPECT_SHA256" ]; then
    echo "MATCH   $p  $got  ($PROGRAM_ID @ slot $DEPLOY_SLOT)"
  else
    echo "DIFFER  $p  got $got  expected $EXPECT_SHA256"; return 1
  fi
}

[ $# -ge 1 ] || { echo "usage: $0 <program>|all" >&2; exit 2; }
if [ "$1" = all ]; then
  rc=0
  for c in "$ROOT"/programs/*/build.conf; do build_one "$(basename "$(dirname "$c")")" || rc=1; done
  exit $rc
fi
for p in "$@"; do build_one "$p"; done
