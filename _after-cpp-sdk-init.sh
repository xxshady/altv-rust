#!/bin/bash
cd altv_sdk/cpp-sdk/version
./get-version.sh
cp version.h ../../cpp-sdk-version.h
cd ../../../cpp_codegen
cargo run
