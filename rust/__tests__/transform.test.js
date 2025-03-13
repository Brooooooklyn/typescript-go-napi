import { test } from 'node:test'
import { strictEqual } from 'node:assert'

import { transform } from '../index.js'

test('transform', () => {
  const result = transform('console.log("Hello, world!");', '/test.ts');
  strictEqual(result, 'console.log("Hello, world!");\n');
});
