import subprocess
import base64

VERSION = "0.1.0"

subprocess.run(["cargo", "build", "--target=wasm32-unknown-unknown", "--release"])

with open('target/wasm32-unknown-unknown/release/outer_noise.wasm', 'rb') as f:
	s = base64.b64encode(f.read()).decode()

with open('glue.js') as f:
	s2 = f.read().replace('{{__wasm_module__}}', s)

with open('dst/index.js', 'w') as f:
	f.write(s2)

with open('dst/package.json', 'w') as f:
	f.write("""{{
  "name": "outer-noise",
  "version": "{VERSION}",
  "main": "index.js",
  "keywords": ["perlin", "noise", "terrain", "wasm"],
  "author": "BlobTheKat",
  "license": "MIT",
  "description": ""
}}""".format(VERSION=VERSION))