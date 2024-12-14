#!/bin/sh

remove_key="${ENV_REMOVE_KEY}"

test 0 -lt ${#remove_key} || exec sh -c 'echo ENV_REMOVE_KEY not set.; exit 1'

native(){
	ENV_REMOVE_KEY="${remove_key}" \
		./rs-json-shortener |
		wc -l
}

runtime_wazero(){
	wazero \
		run \
		-env ENV_REMOVE_KEY="${remove_key}" \
		./rs-json-shortener.wasm |
		wc -l
}

runtime_wasmtime(){
	wasmtime \
		run \
		--env ENV_REMOVE_KEY="${remove_key}" \
		./rs-json-shortener.wasm |
		wc -l
}

runtime_wasmer(){
	wasmer \
		run \
		--env ENV_REMOVE_KEY="${remove_key}" \
		./rs-json-shortener.wasm |
		wc -l
}

compare_jq(){
	jq -c ".${remove_key} = null" |
	wc -l
}

compare_jaq(){
	jaq -c ".${remove_key} = null" |
	wc -l
}

run(){
	native
	#runtime_wazero
	#runtime_wasmer
	#runtime_wasmtime
	
	#compare_jq
	#compare_jaq
}

cat /dev/stdin | run
