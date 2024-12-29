# createHelloWorld Function

This repository contains the implementation of a simple function, `createHelloWorld`, which returns a new function that always returns the string `"Hello World"`, regardless of the arguments passed to it.

## Function Description

### `createHelloWorld()`

The `createHelloWorld` function creates and returns a new function. The returned function accepts any number of arguments and always returns the string `"Hello World"`.

### Example

```typescript
const f = createHelloWorld();
console.log(f()); // "Hello World"
console.log(f({}, null, 42)); // "Hello World"
