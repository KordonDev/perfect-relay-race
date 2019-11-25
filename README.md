# Perfect Relay Race

An app to get the perfect distance for multiple runner in a relay race, where you can choose who is running which distance.

The data is gethered via web application and the calculation is done in rust via webassembly.

## Sources
https://rustwasm.github.io/docs/wasm-bindgen/reference/arbitrary-data-with-serde.html
https://rustwasm.github.io/book/introduction.html

### Commands
`wasm-pack build`
`npm start`

### Data Structure
```
{
    name: String;
    runs: {
        distance: u32;
        duration: u32;
    }[]
}[]
```
