import test from 'ava'
import path from 'path'
import { fileURLToPath } from 'url'
import { dirname } from 'path'

const __filename = fileURLToPath(import.meta.url)
const __dirname = dirname(__filename)

import { gen } from '../index'

test('sync function from native code', (t) => {
  gen({
    path: path.resolve(__dirname, './idl'),
  })
  t.is(1, 1)
})
