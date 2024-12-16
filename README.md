# Outer-noise

```sh
python3 build.py

# Generates img.png testing the noise generator on a large area
deno -A test.js [seed] [x] [y] [w] [h]
# Example:
deno -A test.js "creashaks organzine" -32 -10 64 20
```
![Example noise output from test.js](https://i.imgur.com/SGWoVyj.png)