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
    begin = new Date
    [r,w,h] = await imgJxl(
      readFileSync join ROOT, '1.png'
      'png'
      1.0
    )
    console.log {w,h}, new Date - begin
    write(
      join(ROOT, '1.jxl')
      r
    )
    # t.is(w,1000)
    # t.is(h,667)
    t.true(r instanceof Buffer)
    # t.pass()
    return
)
