import { Bench } from 'tinybench'
import path from 'path'
import { fileURLToPath } from 'url';
import { dirname } from 'path';

import { gen } from '../index.js'

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const b = new Bench()

b.add('Gen ts using rust', () => {
  gen({
    path: path.resolve(__dirname, '../__test__/idl'),
  })
})

await b.run()

console.table(b.table())
