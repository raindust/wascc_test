#!/bin/sh

wascap sign target/wasm32-unknown-unknown/debug/hellohttp.wasm hello_signed.wasm -u ./module.nk -i ./account.nk -n "Hello World" -s -g
