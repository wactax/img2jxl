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
    r = await imgJxl(
      readFileSync join ROOT, '1.jpeg'
      'jpeg'
      1.0
    )
    write(
      join(ROOT, '1.jxl')
      r
    )
    t.true(r instanceof Buffer)
    # t.pass()
    return
)
