const {instance: {exports}, module} = await WebAssembly.instantiate(Uint8Array.from(atob('{{__wasm_module__}}'), c => c.charCodeAt()), {console})
export const seed = new Int32Array(8)
const mem = new DataView(exports.memory.buffer), surf = new Int16Array(exports.memory.buffer, +exports.surfaces, 1)
const sd = +exports.seed, off = +exports.offsets
const ch = new Uint8Array(exports.memory.buffer, +exports.chunk, 512), ch2 = new Uint8Array(exports.memory.buffer, exports.chunk + 512, 256)
export const chunk = new Int32Array(exports.memory.buffer, +exports.chunk2 + 768, 4096)
const chunk2 = new Int32Array(chunk.buffer, +exports.chunk2, 384)

export function genNoise(cb, x, y, localSeed = 0, p = 6, r = 0.5){
	for(let yi=0,j=off;yi<65;yi+=16) for(let xi=0;xi<65;xi+=16,j+=4) mem.setFloat32(j, cb(x+xi, y+yi), true)
	exports.fillNoise(x, y, localSeed, p, r)
	return ch
}
export function genNoisev(arr, x, y, localSeed = 0, p = 6, r = 0.5){
	for(let j=0;j<25;j++) mem.setFloat32(off+(j<<2), arr[j], true)
	exports.fillNoise(x, y, localSeed, p, r)
	return ch
}

export function expand(x, y, localSeed = 0, layers0, layers1, noise, noiseUp, noiseDown){
	ch.set(noise)
	ch2.set(noiseUp)
	ch2.set(noiseDown, 128)
	chunk2.set(layers0)
	chunk2.set(layers1, 192)
	surf[0] = 1
	const c = exports.expand(x, y, localSeed)
	return new Int16Array(surf.buffer, surf.byteOffset, c)
}

const enc = new TextEncoder(), {imul} = Math
export function setSeed(str){
	if(typeof str == 'object'){
		for(let i=0,j=sd;i<8;i++,j+=4) mem.setInt32(j, str[i], true)
		return
	}
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