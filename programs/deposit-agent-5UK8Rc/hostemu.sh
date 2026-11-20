#!/usr/bin/env bash
H=${HOSTEMU_HOST:-aarch64-apple-darwin}
R="$1"; shift
if [ "$#" -eq 1 ] && [ "$1" = "-vV" ]; then
  out=$("$R" -vV) || exit $?
  printf '%s\n' "$out" | sed "s/^host: .*/host: $H/"; exit 0
fi
cfg=0; tgt=0
for a in "$@"; do case "$a" in --print=cfg) cfg=1;; --target|--target=*) tgt=1;; esac; done
if [ $cfg = 1 ] && [ $tgt = 0 ]; then
  in=$(cat)
  real=$(printf '%s' "$in" | "$R" "$@") || exit $?
  emu=$(printf '%s' "$in" | "$R" "$@" --target "$H" 2>/dev/null) || { echo "hostemu: cfg probe for $H failed" >&2; exit 1; }
  printf '%s\n' "$real" | awk '{print} $0=="___"{n++; if(n==2) exit}'
  printf '%s\n' "$emu"  | awk 'f{print} $0=="___"{n++; if(n==2) f=1}'
  exit 0
fi
exec "$R" "$@"
