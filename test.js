import fs from 'node:fs'
import { createCanvas } from "https://deno.land/x/canvas@v1.4.2/mod.ts"
import { getNoise, setSeed, getSeedHash } from './dst/index.js'

setSeed(process.argv[2] || '')
console.log("Seedhash: \x1b[33m%s\x1b[m", getSeedHash())

const startX = +(process.argv[3] ?? -32), startY = +(process.argv[4] ?? -16), width = +process.argv[5] || 64, height = +process.argv[6] || 32

const canvas = createCanvas(width<<6, height<<6)
const ctx = canvas.getContext('2d')
const img = new ImageData(canvas.width, canvas.height)
img.data[3] = 255
const data = new Int32Array(img.data.buffer)
const BLACK = data[0], WHITE = -1
let time = 0
const off = new Float32Array(25)
const s = performance.now()
for(let x = startX; x < startX+width; x++) for(let y = startY; y < startY+height; y++){
	const s = performance.now()
	for(let i = 0, j = 0; i < 5; i++, j += 5)
		off[j] = off[j+1] = off[j+2] = off[j+3] = off[j+4] = ((y<<2)+i) * -.1
	const ch = getNoise(off, x<<6, y<<6, 0)
	time += performance.now() - s
	let offset = (x-startX<<6)+((startY+height-y<<12)-64)*width
	let black = y < 0 ? 0xFFFF4400 : BLACK
	for(let i=0; i < 64; i++){
		let a = ch[i<<1], b = ch[i<<1|1]
		for(let j=0;j<32;j++) data[offset+j] = a>>j&1 ? WHITE : black, data[offset+32+j] = b>>j&1 ? WHITE : black
		offset -= width<<6
	}
}

console.log('\nBuilt \x1b[32m%d\x1b[m chunks in \x1b[35m%dms\x1b[m\n  Time spent in wasm per 1000 chunks: \x1b[35m%sms\x1b[m\n', width*height, performance.now()-s, (time/(width*height*.001)).toFixed(3))

const s2 = performance.now()
ctx.putImageData(img, 0, 0)
const buf = canvas.toBuffer({ type: 'image/png' })
fs.writeFileSync('result.png', buf)
canvas.dispose()
console.log('Saved result.png (\x1b[32m%dKiB\x1b[m) in \x1b[35m%dms\x1b[m', (buf.byteLength/1024).toFixed(2), performance.now()-s2)