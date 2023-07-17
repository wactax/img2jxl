[‼️]: ✏️README.mdt

# @w5/img2jxl

## Install

```
pnpm i -g @w5/img2jxl
```

## Test

[coffee/main.coffee](./coffee/main.coffee) :

```coffee
#!/usr/bin/env coffee

> ../index.js:imgJxl
  ava:test
  path > join dirname
  @w5/uridir
  @w5/write
  # @w5/read
  fs > readFileSync

ROOT = dirname uridir import.meta

test(
  'img → jxl'
  (t) =>
    [r,w,h] = await imgJxl(
      readFileSync join ROOT, '1.jpeg'
      'jpeg'
      1.0
    )
    console.log {w,h}
    write(
      join(ROOT, '1.jxl')
      r
    )
    t.is(w,1000)
    t.is(h,667)
    t.true(r instanceof Buffer)
    # t.pass()
    return
)
```

output :

```

  ✔ img → jxl (431ms)
  ─

  1 test passed
```
