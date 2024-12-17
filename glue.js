const {instance: {exports}, module} = await WebAssembly.instantiate(Uint8Array.from(atob('{{__wasm_module__}}'), c => c.charCodeAt()))
export const seed = new Int32Array(8)
const mem = new Uint8Array(exports.memory.buffer)
const sd = +exports.seed, off = +exports.offsets
const ch = new Uint8Array(exports.memory.buffer, +exports.chunk, 128)

export function genNoise(offs, x, y, localSeed = 0){
	for(let i=0,j=off;i<25;i++,j+=2){
		const k = offs[i]
		mem[j] = k, mem[j+1] = k>>8
	}
	exports.fillNoise(x, y, localSeed)
	return ch
}

export const fillOffsets = (cb, arr = new Int16Array(25)) => {
	for(let x=0,i=0;x<65;x+=16) for(let y=0;y<65;y+=16,i++){
		const v = cb(x,y)
		arr[i] = v<-7.99987?-32767:v>7.99987?32767:round(v*4096)
	}
}

const enc = new TextEncoder(), {imul,round} = Math
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
	for(let i=0,j=sd;i<8;i++,j+=4){
		const k = seed[i]
		mem[j] = k; mem[j+1] = k>>8
		mem[j+2] = k>>16; mem[j+3] = k>>24
	}
}

export function getSeedHash(){
	let x = ''
	for(const i of seed) x += (i>>>0).toString(16).padStart(8, '0')
	return x
}