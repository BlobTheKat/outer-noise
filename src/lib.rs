#![allow(non_upper_case_globals)]

#[export_name="chunk"]
pub static mut chunk: [u64; 96] = [0; 96];

#[export_name="surfaces"]
pub static mut surfaces: [u16; 3072] = [0; 3072];

#[export_name="seed"]
pub static mut seed: [u32; 8] = [0; 8];

#[export_name="chunk2"]
pub static mut chunk2: [i32; 4288] = [0; 4288];

#[export_name="offsets"]
pub static mut p16: [f32; 25] = [0.0; 25];

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

fn utof(a: u32, scale: f32) -> f32 { (a as i32 as f32) * (scale / 2147483648.0) }

// Cubic lerp 3x² - 2x³
fn lerp(a: f32, b: f32, x: f32) -> f32 { a + (b-a) * (3.0-2.0*x)*x*x }
// Linear lerp x
//fn lerp(a: f32, b: f32, x: f32) -> f32 { a + (b-a) * x }
// Test lerp step(x, 0.5)
//fn lerp(a: f32, b: f32, x: f32) -> f32 { if x >= 0.5 { b } else { a } }

#[export_name="fillNoise"]
pub unsafe fn fill_noise(x: u32, y: u32, sd: u32) {
	let x0 = (x-96&0xFFFFFF00)+96;
	let y0 = (y-96&0xFFFFFF00)+96;
	let s = hash2(seed[0], sd);
	let p256_00 = utof(hash3(s, x0, y0), 0.5);
	let p256_10 = utof(hash3(s, x0+256, y0), 0.5);
	let p256_01 = utof(hash3(s, x0, y0+256), 0.5);
	let p256_11 = utof(hash3(s, x0+256, y0+256), 0.5);
	let x256 = (x-x0 >> 6) as f32;
	let y256 = (y-y0 >> 6) as f32;

	let x0 = (x-32&0xFFFFFF80)+32;
	let y0 = (y-32&0xFFFFFF80)+32;
	let s = hash2(seed[1], sd);
	let p128_00 = utof(hash3(s, x0, y0), 0.25);
	let p128_10 = utof(hash3(s, x0+128, y0), 0.25);
	let p128_01 = utof(hash3(s, x0, y0+128), 0.25);
	let p128_11 = utof(hash3(s, x0+128, y0+128), 0.25);
	let x128 = (x-x0 >> 6) as f32;
	let y128 = (y-y0 >> 6) as f32;

	let s = hash2(seed[2]^seed[0], sd);
	let p64_00 = utof(hash3(s, x, y), 0.125);
	let p64_10 = utof(hash3(s, x+64, y), 0.125);
	let p64_01 = utof(hash3(s, x, y+64), 0.125);
	let p64_11 = utof(hash3(s, x+64, y+64), 0.125);

	let s = hash2(seed[2]^seed[1], sd);
	let mut p32: [f32; 9] = [0.0; 9];
	for xd in 0..3u32 { for yd in 0..3u32 {
		p32[(xd+yd*3) as usize] = utof(hash3(s, x + (xd<<5), y + (yd<<5)), 0.0625);
	} }

	let s = hash2(seed[3]^seed[0], sd);
	for xd in 0..5u32 { for yd in 0..5u32 {
		p16[(xd+yd*5) as usize] += utof(hash3(s, x + (xd<<4), y + (yd<<4)), 0.03125);
	} }

	let s = hash2(seed[3]^seed[1], sd);
	let mut p8: [f32; 81] = [0.0; 81];
	for xd in 0..9u32 { for yd in 0..9u32 {
		p8[(xd+yd*9) as usize] = utof(hash3(s, x + (xd<<3), y + (yd<<3)), 0.015625);
	} }

	let s = hash2(seed[3]^seed[2]^seed[0], sd);
	let mut p4: [f32; 289] = [0.0; 289];
	for xd in 0..17u32 { for yd in 0..17u32 {
		p4[(xd+yd*17) as usize] = utof(hash3(s, x + (xd<<2), y + (yd<<2)), 0.0078125);
	} }
	
	let mut y = 0;
	let mut yf = 0f32;
	while y < 64{
		let mut x = 0;
		let mut xf = 0f32;
		let mut line: u64 = 0;
		let p256_0 = lerp(p256_00, p256_01, (y256+yf)*0.25);
		let p256_1 = lerp(p256_10, p256_11, (y256+yf)*0.25);
		let p128_0 = lerp(p128_00, p128_01, (y128+yf)*0.5);
		let p128_1 = lerp(p128_10, p128_11, (y128+yf)*0.5);
		let p64_0 = lerp(p64_00, p64_01, yf);
		let p64_1 = lerp(p64_10, p64_11, yf);
		while x < 64 {
			let base = lerp(p256_0, p256_1, (x256+xf)*0.25)
				+ lerp(p128_0, p128_1, (x128+xf)*0.5)
				+ lerp(p64_0, p64_1, xf);
			xf += 0.015625;
			let i = (x>>5) + (y>>5)*3;
			let yf = (y&31) as f32 * 0.03125;
			let base = base + lerp(lerp(p32[i], p32[i+3], yf), lerp(p32[i+1], p32[i+4], yf), (x&31) as f32 * 0.03125);
			let i = (x>>4) + (y>>4)*5;
			let yf = (y&15) as f32 * 0.0625;
			let base = base + lerp(lerp(p16[i], p16[i+5], yf), lerp(p16[i+1], p16[i+6], yf), (x&15) as f32 * 0.0625);
			let i = (x>>3) + (y>>3)*9;
			let yf = (y&7) as f32 * 0.125;
			let base = base + lerp(lerp(p8[i], p8[i+9], yf), lerp(p8[i+1], p8[i+10], yf), (x&7) as f32 * 0.125);
			let i = (x>>2) + (y>>2)*17;
			let yf = (y&3) as f32 * 0.25;
			let base = base + lerp(lerp(p4[i], p4[i+17], yf), lerp(p4[i+1], p4[i+18], yf), (x&3) as f32 * 0.25);

			if base > 0.0 { line |= 1<<x; }
			x += 1; 
		}
		chunk[y] = line;
		y += 1; yf += 0.015625;
	}
}

unsafe fn add_surfaces(swap_bytes: bool, sfi: &mut usize, y: u32, l: u64){
	let mut l = l;
	if swap_bytes {
		while l != 0 {
			let x = l.trailing_zeros();
			l &= !(1<<x);
			surfaces[*sfi] = ((x | y<<6) as u16).swap_bytes();
			*sfi += 1;
		}
	}else{
		while l != 0 {
			let x = l.trailing_zeros();
			l &= !(1<<x);
			surfaces[*sfi] = (x | y<<6) as u16;
			*sfi += 1;
		}
	}
}

#[export_name="expand"]
pub unsafe fn expand(cx: u32, cy: u32, sd: u32) -> i32{
	let mut last: u64 = 0xFFFF_FFFF_FFFF_FFFF;
	let mut y = 80u32;
	let mut sfi = 0;
	let swap_bytes = surfaces[0] != 1;
	let s = hash2(seed[3]^seed[2]^seed[1], sd);
	while y > 64 {
		y -= 1;
		let line = chunk[y as usize];
		add_surfaces(swap_bytes, &mut sfi, y, line & !last);
		last = line;
	}
	let mut j = 192;
	while y > 0 {
		y -= 1; j -= 3;
		let yu = y as usize;
		let line = chunk[yu];
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
		add_surfaces(swap_bytes, &mut sfi, y, line & !last);
		last = line;
	}
	y = 96;
	while y > 80 {
		y -= 1;
		let line = chunk[y as usize];
		add_surfaces(swap_bytes, &mut sfi, y-96, line & !last);
		last = line;
	}
	sfi as i32
}