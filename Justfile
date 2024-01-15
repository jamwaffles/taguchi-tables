build *args:
    wasm-pack build --target web --out-dir www/pkg --no-pack {{args}}
    cd www && npm run build

serve:
    cd www && npm run serve

clean:
    cargo clean
    rm -rf www/.parcel-cache
    rm -rf www/node_modules
    rm -rf pkg
