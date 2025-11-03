# Code Style Guidelines

## TypeScript

- Use proper type annotations for function parameters and return values
- For simple loops prefer a traditional `for` keyword or `for of` pattern over the `forEach` method

## React

- Import React as a namespace: `import React from "react"`
- If both React and JSX are imported use: `import React, { type JSX } from "react"`
- Access React hooks through the React namespace: `React.useState`, `React.useEffect`, etc.

## Formatting

- Use consistent indentation
- Always include curly braces for control flow statements (if, for, while, etc.)

## Imports

- `@raiment-*` is an alias for local `modules/raiment-*` packages, always use that alias for local packages
- All imports from the same module should be grouped together in a single import statement
- List all external imports before `@raiment-*` imports local imports
