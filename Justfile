build *args: (build-rust args)
    cd www && npm run build

build-rust *args:
    wasm-pack build --target web --out-dir www/pkg --no-pack {{args}}

watch:
    cargo watch -s 'just build-rust'


serve:
    cd www && npm run serve

clean:
    cargo clean
    rm -rf www/.parcel-cache
    rm -rf www/node_modules
    rm -rf pkg
