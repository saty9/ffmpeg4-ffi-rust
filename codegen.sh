#!/bin/sh
export FF_DO_CODEGEN=1
cargo "$@" check
