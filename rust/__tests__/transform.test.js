// @ts-check

import { test } from 'node:test'
import { strictEqual } from 'node:assert'

import { transpileModule } from '../index.js'

test('transpileModule', () => {
  const result = transpileModule('console.log("Hello, world!");', {
    filename: '/test.ts',
  });
  strictEqual(result, 'console.log("Hello, world!");\n');
});
