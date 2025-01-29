#![allow(non_upper_case_globals)]

#[repr(C)]
pub struct Chunk{
	blocks: [u64; 64],
	surfaces: [u16; 2048]
}

#[export_name="chunk"]
pub static mut chunk: Chunk = Chunk {
	blocks: [0; 64], surfaces: [0; 2048]
};

#[export_name="seed"]
pub static mut seed: [u32; 8] = [0; 8];

#[export_name="chunk2"]
pub static mut chunk2: [i32; 4288] = [0; 4288];

#[export_name="offsets"]
pub static mut offsets: [f32; 25] = [0.0; 25];

// Quick bijective hash
fn hash2(a: u32, b: u32) -> u32 {
	let x = a ^ (b * 1597334673);
	let x = x * 0x7feb352d;
	let x = x ^ (x >> 15);
	x * 0x846ca68b
}
fn hash3(a: u32, b: u32, c: u32) -> u32 {
	let x = a ^ (b * 1597334673);
	let x = x * 0x7feb352d;
	let x = x ^ (x >> 15) ^ (c * 3812015801);
	let x = x * 0xbce6059f;
	let x = x ^ (x >> 8);
	x * 0x846ca68b
}

fn utof(a: u32) -> f32 { (a as i32 as f32) * (1.0 / 2147483648.0) }

// Cubic lerp 3x² - 2x³
fn lerp(a: f32, b: f32, x: f32) -> f32 { a + (b-a) * (3.0-2.0*x)*x*x }
// Linear lerp x
fn lerp1(a: f32, b: f32, x: f32) -> f32 { a + (b-a) * x }
// Test lerp step(x, 0.5)
//fn lerp(a: f32, b: f32, x: f32) -> f32 { if x >= 0.5 { b } else { a } }

#[export_name="fillNoise"]
pub unsafe fn fill_noise(x: u32, y: u32, sd: u32, period: f32, roughness: f32) -> i32 {
	let swap_bytes = chunk.surfaces[0] != 1;
	let mut sc = 1.0;
	let mut p = period;
	while p > 7.0{
		let p1 = p.ceil()-1.0;
		let sc1 = p - p1; p = p1;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		let k = 2u32<<(p1 as u32);
		let s: u32 = hash2(seed[0]^(k|k>>3), sd);
		let x0 = x&(-(k as i32) as u32);
		let y0 = y&(-(k as i32) as u32);
		let inv = 16.0/(k as f32);

		let p00 = utof(hash3(s, x0, y0)) * scl;
		let p10 = utof(hash3(s, x0+k, y0)) * scl;
		let p01 = utof(hash3(s, x0, y0+k)) * scl;
		let p11 = utof(hash3(s, x0+k, y0+k)) * scl;
		let mut xf0 = [0f32; 5];
		let mut xf1 = [0f32; 5];
		let mut xf = (x-x0>>4) as f32 * inv;
		for i in 0..5{
			xf0[i] = lerp(p00, p10, xf);
			xf1[i] = lerp(p01, p11, xf);
			xf += inv;
		}
		let yf = (y-y0>>4) as f32 * inv;
		for x in 0..5 {
			let xf0 = xf0[x];
			let xf1 = xf1[x];
			let mut yf = yf;
			for y in 0..5{
				offsets[x+y*5] += lerp(xf0, xf1, yf);
				yf += inv;
			}
		}
	}

	let x0 = x&0xFFFFFF80;
	let y0 = y&0xFFFFFF80;
	let s = hash2(seed[1], sd);
	let (p128_00, p128_10, p128_01, p128_11) = if p > 6.0{
		let sc1 = p-6.0; p = 6.0;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		let a = utof(hash3(s, x0, y0)) * scl;
		let b = utof(hash3(s, x0+128, y0)) * scl;
		let c = utof(hash3(s, x0, y0+128)) * scl;
		let d = utof(hash3(s, x0+128, y0+128)) * scl;
		(a, b, c, d)
	} else { (0.0, 0.0, 0.0, 0.0) };
	let x128 = (x-x0 >> 6) as f32;
	let y128 = (y-y0 >> 6) as f32;

	let s = hash2(seed[2]^seed[0], sd);
	let (p64_00, p64_10, p64_01, p64_11) = if p > 5.0{
		let sc1 = p-5.0; p = 5.0;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		let a = utof(hash3(s, x, y)) * scl;
		let b = utof(hash3(s, x+64, y)) * scl;
		let c = utof(hash3(s, x, y+64)) * scl;
		let d = utof(hash3(s, x+64, y+64)) * scl;
		(a, b, c, d)
	} else { (0.0, 0.0, 0.0, 0.0) };

	let s = hash2(seed[2]^seed[1], sd);
	let mut p32: [f32; 9] = [0.0; 9];
	if p > 4.0{
		let sc1 = p-4.0; p = 4.0;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		for xd in 0..3u32 { for yd in 0..3u32 {
			p32[(xd+yd*3) as usize] = utof(hash3(s, x + (xd<<5), y + (yd<<5))) * scl;
		} }
	}

	let s = hash2(seed[3]^seed[0], sd);
	let mut p16: [f32; 25] = [0.0; 25];
	if p > 3.0{
		let sc1 = p-3.0; p = 3.0;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		for xd in 0..5u32 { for yd in 0..5u32 {
			p16[(xd+yd*5) as usize] = utof(hash3(s, x + (xd<<4), y + (yd<<4))) * scl;
		} }
	}

	let s = hash2(seed[3]^seed[1], sd);
	let mut p8: [f32; 81] = [0.0; 81];
	if p > 2.0{
		let sc1 = p-2.0; p = 2.0;
		let scl = sc1*sc; sc *= lerp1(1.0, roughness, sc1);
		for xd in 0..9u32 { for yd in 0..9u32 {
			p8[(xd+yd*9) as usize] = utof(hash3(s, x + (xd<<3), y + (yd<<3))) * scl;
		} }
	}

	let s = hash2(seed[3]^seed[2]^seed[0], sd);
	let mut p4: [f32; 289] = [0.0; 289];
	if p > 1.0{
		sc *= p-1.0;
		for xd in 0..17u32 { for yd in 0..17u32 {
			p4[(xd+yd*17) as usize] = utof(hash3(s, x + (xd<<2), y + (yd<<2))) * sc;
		} }
	}
	
	let mut y = 0;
	let mut yf = 0.0078125f32;
	let mut last: u64 = 0;
	let mut sfi: usize = 0;
	while y < 64{
		let mut x = 0;
		let mut xf = 0.0078125f32;
		let mut line: u64 = 0;
		let p128_0 = lerp(p128_00, p128_01, (y128+yf)*0.5);
		let p128_1 = lerp(p128_10, p128_11, (y128+yf)*0.5);
		let p64_0 = lerp(p64_00, p64_01, yf);
		let p64_1 = lerp(p64_10, p64_11, yf);
		while x < 64 {
			let base = lerp(p128_0, p128_1, (x128+xf)*0.5) + lerp(p64_0, p64_1, xf);
			xf += 0.015625;
			let i = (x>>5) + (y>>5)*3;
			let yf = (y&31) as f32 * 0.03125;
			let base = base + lerp(lerp(p32[i], p32[i+3], yf), lerp(p32[i+1], p32[i+4], yf), (x&31) as f32 * 0.03125);
			let i = (x>>4) + (y>>4)*5;
			let yf = (y&15) as f32 * 0.0625;
			let xf1 = (x&15) as f32 * 0.0625;
			let base = base + lerp(lerp(p16[i], p16[i+5], yf), lerp(p16[i+1], p16[i+6], yf), xf1)
				+ lerp1(lerp1(offsets[i], offsets[i+5], yf), lerp1(offsets[i+1], offsets[i+6], yf), xf1);
			let i = (x>>3) + (y>>3)*9;
			let yf = (y&7) as f32 * 0.125;
			let base = base + lerp(lerp(p8[i], p8[i+9], yf), lerp(p8[i+1], p8[i+10], yf), (x&7) as f32 * 0.125);
			let i = (x>>2) + (y>>2)*17;
			let yf = (y&3) as f32 * 0.25;
			let base = base + lerp(lerp(p4[i], p4[i+17], yf), lerp(p4[i+1], p4[i+18], yf), (x&3) as f32 * 0.25);

			if base > 0.0 { line |= 1<<x; }
			x += 1; 
		}
		chunk.blocks[y] = line;
		let l = !line&last;
		add_surfaces(swap_bytes, &mut sfi, y, l);
		last = line;
		y += 1; yf += 0.015625;
	}
	sfi as i32
}

unsafe fn add_surfaces(swap_bytes: bool, sfi: &mut usize, y: usize, l: u64){
	let mut l = l;
	let y = (y as u16) << 6;
	if swap_bytes {
		while l != 0 {
			let x = l.trailing_zeros() as u16;
			l &= !(1<<x);
			chunk.surfaces[*sfi] = (x | y).swap_bytes();
			*sfi += 1;
		}
	}else{
		while l != 0 {
			let x = l.trailing_zeros() as u16;
			l &= !(1<<x);
			chunk.surfaces[*sfi] = (x | y) as u16;
			*sfi += 1;
		}
	}
}

#[export_name="expand"]
pub unsafe fn expand(cx: u32, cy: u32, sd: u32){
	let mut y = 64;
	let s = hash2(seed[3]^seed[2]^seed[1], sd);
	let mut j = 192;
	while y > 0 {
		y -= 1; j -= 3;
		let yu = y as usize;
		let line = chunk.blocks[yu];
		let c = &mut chunk2[yu+3<<6 .. yu+4<<6];
		let b0 = chunk2[j+1];
		let p = chunk2[j] as u8;
		let b1 = chunk2[j+193];
		let q = chunk2[j+192] as u8;
		if (p|q) == 0 {
			let b1a = b1^b0;
			for x in 0..64 {
				c[x as usize] = b0 ^ (-((line>>x&1) as i32) & b1a);
			}
		}else{
			let b2a = chunk2[j+2]^b0;
			let b3a = chunk2[j+194]^b1;
			for x in 0..64 {
				let rand = hash3(s, cx+x, cy+y) as u8;
				c[x as usize] = if (line>>x&1)==0 {
					b0 ^ (-((rand<p) as i32)&b2a)
				}else{
					b1 ^ (-((rand<q) as i32)&b3a)
				}
			}
		}
	}
}

pub struct Biome{
	temp: f32, humd: f32,
	i_prio: f32, block: i32,
	count: usize, next: usize
}

#[export_name="findBiome"]
pub unsafe fn find_biome(arr: *const Biome, base: usize, temp: f32, humd: f32, block: i32) -> usize{
	let mut base = base;
	loop {
		let cur = &*(arr.add(base));
		let count = cur.count;
		if count == 0{ return base; }
		let mut best_dist = f32::INFINITY;
		let mut best_biome = base;
		for i in cur.next..(cur.next+count) {
			let n2 = &*arr.add(i);
			if n2.block != 2147483647 && n2.block != block { continue; }
			let dx = n2.temp - temp;
			let dy = n2.humd - humd;
			let dist_sq = (dx*dx + dy*dy) * n2.i_prio;
			if dist_sq <= best_dist {
				best_dist = dist_sq;
				best_biome = i;
			}
		}
		if best_biome == base{ return best_biome; }
		base = best_biome;
	}
}