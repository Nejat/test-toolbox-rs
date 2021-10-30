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
