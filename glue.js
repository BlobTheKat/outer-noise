const {instance: {exports}, module} = await WebAssembly.instantiate(Uint8Array.from(atob('{{__wasm_module__}}'), c => c.charCodeAt()))
export const seed = new Int32Array(8)
const mem = new DataView(exports.memory.buffer)
const sd = exports.seed>>2, off = exports.offsets>>2
const ch = new Uint8Array(exports.memory.buffer, +exports.chunk, 128)

export function genNoise(offs, x, y, localSeed = 0){
	for(let i=0;i<25;i++) mem.setFloat32((off+i)<<2, offs[i], true)
	exports.fillNoise(x, y, localSeed)
	return ch
}

const enc = new TextEncoder(), {imul} = Math
export function setSeed(str){
	if(str instanceof ArrayBuffer) return void(sd.set(new Int32Array(sd, 0, 8)))
	const arr = enc.encode(str+'\0')
	seed.fill(0)
	let x = 0xe336beb9|0, i = 0
	let coeff = 1597334673
	// Quick bijective hash
	for(; i < arr.length; i += 4){
		const y = arr[i]<<24|arr[i+1]<<16|arr[i+2]<<8|arr[i+3]
		x = imul(x ^ imul(y, coeff), 0x7feb352d)
		x ^= x >> 15
		coeff += 0x4319fa62
		seed[(i>>2)&7] ^= x
	}
	let j = i >>= 2; coeff = 1
	do{
		x = imul(x, coeff += 0x6eeb828a)
		x ^= x >> 15
		seed[j = j+1&7] ^= x
	}while(j != i)
	for(let i=0;i<8;i++) mem.setInt32((sd+i)<<2, seed[i], true)
}

export function getSeedHash(){
	let x = ''
	for(const i of seed) x += (i>>>0).toString(16).padStart(8, '0')
	return x
}