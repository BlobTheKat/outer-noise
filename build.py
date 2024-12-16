import subprocess
import base64

subprocess.run(["cargo", "build", "--target=wasm32-unknown-unknown", "--release"])

with open('target/wasm32-unknown-unknown/release/outer_noise.wasm') as f:
	s = base64.b64encode(f.read())

with open('glue.js') as f:
	s2 = f.read().format(__wasm_module__=s)

with open('index.js', 'wx') as f:
	f.write(s2)