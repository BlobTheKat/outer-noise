const {instance: {exports}, module} = await WebAssembly.instantiate(Uint8Array.from(atob('{{__wasm_module__}}'), c => c.charCodeAt()))
export const seed = new Int32Array(8)
const mem = new DataView(exports.memory.buffer)
const sd = +exports.seed, off = +exports.offsets
const ch = new Uint8Array(exports.memory.buffer, +exports.chunk, 512)
export const chunk = new Int32Array(exports.memory.buffer, +exports.chunk2, 4096)

export function genNoise(cb, x, y, localSeed = 0){
	for(let yi=0,j=off;yi<65;yi+=16) for(let xi=0;xi<65;xi+=16,j+=4) mem.setFloat32(j, cb(x+xi, y+yi), true)
	exports.fillNoise(x, y, localSeed)
	return ch
}
export function genNoisev(arr, x, y, localSeed = 0){
	for(let j=0;j<25;j++) mem.setFloat32(off+(j<<2), arr[j], true)
	exports.fillNoise(x, y, localSeed)
	return ch
}

export function expand(noise, a = 0, b = 0){
	chunk[0] = a; chunk[1] = b
	ch.set(noise)
	exports.expand()
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
	for(let i=0,j=sd;i<8;i++,j+=4) mem.setInt32(j, seed[i], true)
}

export function getSeedHash(){
	let x = ''
	for(const i of seed) x += (i>>>0).toString(16).padStart(8, '0')
	return x
}