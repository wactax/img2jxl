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
      readFileSync join ROOT, 'logo.webp'
      'webp'
      1.0 # https://docs.rs/jpegxl-rs/latest/jpegxl_rs/encode/struct.JxlEncoderBuilder.html#method.quality
    )
    write(
      join(ROOT, 'logo.jxl')
      r
    )
    t.true(r instanceof Buffer)
    # t.pass()
    return
)
