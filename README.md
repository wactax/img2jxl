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

> ../index.js:svgWebp
  ava:test
  path > join dirname
  @w5/uridir
  @w5/write
  # @w5/read
  fs > readFileSync

ROOT = dirname uridir import.meta

test(
  'svg → webp'
  (t) =>
    r = await svgWebp(
      readFileSync join ROOT, 'logo.svg'
      80
    )
    write(
      join(ROOT, 'logo.webp')
      r
    )
    t.true(r instanceof Buffer)
    return
)
```

output :

```

  ✔ svg → webp (963ms)
  ─

  1 test passed
```
