const {instance: {exports}, module} = await WebAssembly.instantiate(Uint8Array.from(atob('{__wasm_module__}'), c => c.charCodeAt()))
const sd = new Int32Array(exports.memory.buffer, +exports.seed, 8)
const ch = new Int32Array(exports.memory.buffer, +exports.chunk, 128), off = new Float32Array(exports.memory.buffer, +exports.offsets, 25)

export function getNoise(offs, x, y, localSeed = 0){
	off.set(offs)
	exports.fillNoise(x, y, localSeed)
	return ch.slice()
}

const enc = new TextEncoder(), {imul} = Math
export function setSeed(str){
	if(str instanceof ArrayBuffer) return void(sd.set(new Int32Array(sd, 0, 8)))
	const arr = enc.encode(str+'\0')
	sd.fill(0)
	let x = 0xe336beb9|0, i = 0
	let coeff = 1597334673
	// Quick bijective hash
	for(; i < x.length; i += 4){
		const y = arr[i]<<24|arr[i+1]<<16|arr[i+2]<<8|arr[i+3]
		x = imul(x ^ imul(y, coeff), 0x7feb352d)
		x ^= x >> 15
		coeff += 0x4319fa62
		sd[(i>>2)&7] ^= x
	}
	let j = i >>= 2; coeff = 1
	do{
		x = imul(x, coeff += 0x6eeb828a)
		x ^= x >> 15
		sd[j = j+1&7] ^= x
	}while(j != i)
}