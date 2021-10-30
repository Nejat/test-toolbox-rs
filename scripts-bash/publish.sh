#!/bin/bash

IFS='/' read -ra script_parts <<< "$0"

NC='\033[0m'
RED='\033[0;31m'
YELLOW='\033[1;33m'

if [[ "$0" == "./${script_parts[-1]}" ]]
then
  echo -e "\n${YELLOW}Scripts need to be run from the project root${NC}, i.e. ./scripts-ps/${script_parts[-1]}\n"
  exit 1
fi

function confirm-success() {
    exit_code=$?
    if [ $exit_code -ne 0 ]
    then
        echo -e "\n$1 ${RED}failed${NC} with exitcode ${exit_code}\n"
        exit $exit_code
    fi
}

clear

for_reals="false"
skip_clean="false"

while test $# -gt 0; do
  case "$1" in
    --for_reals) for_reals="true" && shift ;;
    --skip_clean) skip_clean="true" && shift ;;
    *) shift ;;
  esac
done

if [[ "${for_reals}" == "true" ]] && [[ "${skip_clean}" == "true" ]]
then
    echo -e "${YELLOW}Can not skip clean step when actually publishing${NC}\n"

    skip_clean="false"
fi

if [[ "${skip_clean}" == "true" ]]
then
    cargo clean
    confirm-success "clean"
fi

echo -e "${YELLOW}running clippy actual unoptimized"
cargo clippy --features="actual"
confirm-success "clippy actual unoptimized"

echo -e "${YELLOW}running clippy actual optimized"
cargo clippy --release --features="actual"
confirm-success "clippy actual optimized"

echo -e "${YELLOW}running clippy capture unoptimized"
cargo clippy --features="capture"
confirm-success "clippy capture unoptimized"

echo -e "${YELLOW}running clippy capture optimized"
cargo clippy --release --features="capture"
confirm-success "clippy capture optimized"

echo -e "${YELLOW}running clippy expected unoptimized"
cargo clippy --features="expected"
confirm-success "clippy expected unoptimized"

echo -e "${YELLOW}running clippy expected optimized"
cargo clippy --release --features="expected"
confirm-success "clippy expected optimized"

echo -e "${YELLOW}running test all features unoptimized"
cargo test --features="all" -- --nocapture --test-threads=1
confirm-success "test all features unoptimized"

echo -e "${YELLOW}running test all features optimized"
cargo test --features="all" --release -- --nocapture --test-threads=1
confirm-success "test all features optimized"

if [[ "${for_reals}" == "true" ]]
then
  echo -e "${YELLOW}publish"
  cargo publish --locked --features="all"
  confirm-success "publish"
else
  echo -e "${YELLOW}publish dry run"
  cargo publish --locked --features="all" --dry-run
  confirm-success "publish dry run"
fi