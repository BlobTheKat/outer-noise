const {exports} = (await WebAssembly.instantiate(Uint8Array.from(atob('{{__wasm_module__}}'), c => c.charCodeAt()))).instance
export const seed = new Int32Array(8)
const mem = new DataView(exports.memory.buffer), surf = new Int16Array(mem.buffer, +exports.surfaces, 1)
const sd = +exports.seed, off = +exports.offsets
const chi = +exports.chunk
const ch = new Uint8Array(mem.buffer, chi, 512)
export const chunk = new Int32Array(mem.buffer, +exports.chunk2 + 768, 4096)
const chunk2 = new Int32Array(chunk.buffer, +exports.chunk2, 384)

export function genNoise(cb, x, y, localSeed = 0, p = 6, r = 0.5){
	for(let yi=0,j=off;yi<65;yi+=16) for(let xi=0;xi<65;xi+=16,j+=4) mem.setFloat32(j, cb(x+xi, y+yi), true)
	const i = exports.fillNoise(x, y, localSeed, p, r)
	return new Uint8Array(mem.buffer, chi, (256+i)<<1)
}
export function genNoisev(arr, x, y, localSeed = 0, p = 6, r = 0.5){
	for(let j=0;j<25;j++) mem.setFloat32(off+(j<<2), arr[j], true)
	const i = exports.fillNoise(x, y, localSeed, p, r)
	return new Uint8Array(mem.buffer, chi, (256+i)<<1)
}

export function expand(x, y, localSeed = 0, layers0, layers1, noise){
	ch.set(noise)
	chunk2.set(layers0)
	chunk2.set(layers1, 192)
	surf[0] = 1
	exports.expand(x, y, localSeed)
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

const biomeBase = +exports.__heap_base
let biomeTip = biomeBase, j = 0
export const addBiome = (t=0,h=0,p=0,b=0,c=0,n=0) => {
	const i = biomeTip
	biomeTip += 24
	if(biomeTip > mem.byteLength) exports.memory.grow(1)
	mem.setFloat32(i, t, true)
	mem.setFloat32(i+4, h, true)
	mem.setFloat32(i+8, 1/(p*p), true)
	mem.setInt32(i+12, b, true)
	mem.setUint32(i+16, c, true)
	mem.setUint32(i+20, n, true)
	return j++
}

export const findBiome = (i=0, t=0, h=0, b=0) => exports.findBiome(biomeBase, i, t, h, b)