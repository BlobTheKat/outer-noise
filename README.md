# Outer-noise

```sh
python3 build.py

# Generates result.png testing the noise generator on a large area
deno -A test.js [seed] [x] [y] [w] [h]
# Example:
deno -A test.js "creashaks organzine" -32 -10 64 20
```
![Example noise output from test.js](https://i.imgur.com/SGWoVyj.png)

# Interface

```js
import { setSeed, genNoise, fillOffsets } from 'outer-noise'

setSeed('random-seed')

// Noise offsets specified at every 16 tiles, which is interpolated by genNoise()
// Negative offsets make 0 (air) more likely and positive offsets make 1 (ground) more likely
// Here we make an offset that looks like
//  32 | -.4 -.4 -.4 -.4 -.4
//  16 | -.2 -.2 -.2 -.2 -.2
//y  0 |  .0  .0  .0  .0  .0
// -16 |  .2  .2  .2  .2  .2
// -32 |  .4  .4  .4  .4  .4
const offsetFn = (x, y) => {
	return (y - 32) * -0.0125
}

// Chunk coordinates
const chX = 0, chY = 0
// local_seed can be used to generate many (2^32) unique noises without changing the main seed
let arr = genNoise(offsetFn, chX, chY, /*local_seed*/ 0)

// Note: arr is only valid until the next call to genNoise(), copy it if necessary!
noiseCache.set(chX, chY, arr = arr.slice())

// arr contains 512 8-bit values arranged left-to-right, bottom-to-top, least-significant-bit first

function getValue(tileX, tileY){
	return arr[tileY<<3|tileX>>3] >> (tileX&7) & 1
}
```