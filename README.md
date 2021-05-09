Wasm Playground

You need python 3 for it to work
````
wasm-pack build --target web
mv pkg static
cd static
python3 -m http.server
````