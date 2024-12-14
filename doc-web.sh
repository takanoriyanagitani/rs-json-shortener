#!/bin/sh

addr=127.0.0.1
port=12138
docd=./target/

miniserve \
	--interfaces "${addr}" \
	--port ${port} \
	"${docd}"
