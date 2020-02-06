#!/bin/sh
export FF_DO_CODEGEN=1
touch src/sys.rs
cargo "$@" check
