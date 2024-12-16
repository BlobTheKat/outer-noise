import fs from 'node:fs'
import { createCanvas } from "https://deno.land/x/canvas@v1.4.2/mod.ts"

const {instance: {exports}, module} = await WebAssembly.instantiate(fs.readFileSync('./target/wasm32-unknown-unknown/release/outer_noise.wasm'))
const startX = +process.argv[2] ?? -32, startY = +process.argv[3] ?? -32, width = +process.argv[4] || 64, height = +process.argv[5] || 64
const ch = new Int32Array(exports.memory.buffer, +exports.chunk, 512), off = new Float32Array(exports.memory.buffer, +exports.offsets, 100)

const c = createCanvas(width<<6, height<<6)
const ctx = c.getContext('2d')
const img = new ImageData(c.width, c.height)
img.data[3] = 255
const data = new Int32Array(img.data.buffer)
const BLACK = data[0], WHITE = -1
let time = 0
const s = performance.now()
for(let x = startX; x < startX+width; x++) for(let y = startY; y < startY+height; y++){
	const s = performance.now()
	for(let i = 0, j = 0; i < 5; i++, j += 5)
		off[j] = off[j+1] = off[j+2] = off[j+3] = off[j+4] = ((y<<2)+i) * -.1
	exports.fillNoise(x<<6, y<<6, 0)
	time += performance.now() - s
	let offset = (x-startX<<6)+((startY+height-y<<12)-64)*width
	let black = y < 0 ? 0xFFFF4400 : BLACK
	for(let i=0; i < 64; i++){
		let a = ch[i<<1], b = ch[i<<1|1]
		for(let j=0;j<32;j++) data[offset+j] = a>>j&1 ? WHITE : black, data[offset+32+j] = b>>j&1 ? WHITE : black
		offset -= width<<6
	}
}

console.log('Built %d chunks in %dms\n  Time spent in wasm per 1000 chunks: %sms', width*height, performance.now()-s, (time/(width*height*.001)).toFixed(3))

ctx.putImageData(img, 0, 0)
fs.writeFileSync('img.png', c.toBuffer({ type: 'image/png' }))
c.dispose()