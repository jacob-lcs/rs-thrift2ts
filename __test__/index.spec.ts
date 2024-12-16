import test from 'ava'
import path from 'path'

import { gen } from '../index'

test('sync function from native code', (t) => {
  gen({
    path: path.resolve('./idl'),
  })
  t.is(1, 1)
})
