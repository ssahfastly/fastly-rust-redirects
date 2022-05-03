fastly compute build && 
    wasm-strip bin/main.wasm && 
    wizer bin/main.wasm -o bin/opto.wasm -r _start=wizer.resume && 
    mv bin/opto.wasm bin/main.wasm && 
    wasm-strip bin/main.wasm && 
    fastly compute pack -w bin/main.wasm && 
    fastly compute serve --skip-build
