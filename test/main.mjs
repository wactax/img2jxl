#!/usr/bin/env -S node --loader=@w5/jsext --trace-uncaught --expose-gc --unhandled-rejections=strict --experimental-import-meta-resolve
var ROOT;

import imgJxl from '../index.js';

import test from 'ava';

import {
  join,
  dirname
} from 'path';

import uridir from '@w5/uridir';

import write from '@w5/write';

import {
  // @w5/read
  readFileSync
} from 'fs';

ROOT = dirname(uridir(import.meta));

test('img → jxl', async(t) => {
  var h, r, w;
  [r, w, h] = (await imgJxl(readFileSync(join(ROOT, '1.jpeg')), 'jpeg', 1.0));
  console.log({w, h});
  write(join(ROOT, '1.jxl'), r);
  t.is(w, 1000);
  t.is(h, 667);
  t.true(r instanceof Buffer);
});

// t.pass()
