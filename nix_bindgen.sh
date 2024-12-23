#!/usr/bin/env zsh

source ~/.zshrc



# echo LIBCLANG_PATH="$(nixpath llvmPackages_12.libclang.lib)/lib"
LIBCLANG_PATH="$(nixpath llvmPackages_12.libclang.lib)/lib" cargo run

