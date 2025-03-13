# `@typescript-go/api`

This project is an experimental Node.js binding for https://github.com/microsoft/typescript-go.

> 🚀 Help me to become a full-time open-source developer by [sponsoring me on Github](https://github.com/sponsors/Brooooooklyn)

We have only few api at this moment:

## Transform code

```ts
import { transform } from '@typescript-go/api'

transform('const a: number = 1;', '/absolute/path/a.ts')

// 'const a = 1;\n'
```

## TypeCheck a project

```ts
import { runProject } from '@typescript-go/api'

// equivalent to `tsc -p ./rust/tsconfig.json`
runProject('./rust/tsconfig.json')
```
