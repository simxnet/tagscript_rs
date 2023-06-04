<h1 align="center">tagscript.rs</h1>
<p>
  <img alt="Version" src="https://img.shields.io/badge/version-0.1.0-blue.svg?cacheSeconds=2592000" />
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/ruunao" target="_blank">
    <img alt="Twitter: ruunao" src="https://img.shields.io/twitter/follow/ruunao.svg?style=social" />
  </a>
</p>

> String interpreter in Rust ([tagscript](https://github.com/imranbarbhuiya/TagScript) port)

## Installation

```sh
cargo add tagscript_rs
```
or add this to your Cargo.toml
```toml
[dependencies]
tagscript_rs = "0.1.0"
```

## Usage
Simple usage example
```rs
fn main() {
    let mut parser = TemplateParser::new("Hello, {{name|uppercase}}!");
    parser.parse();

    let mut data = HashMap::new();
    data.insert("name".to_string(), "world".to_string());

    let result = parser.render(&data);
    println!("{}", result);
}
```

## Documentation
Not yet lol

## Author

👤 **Runa**

* Twitter: [@ruunao](https://twitter.com/ruunao)
* Github: [@ruunao](https://github.com/ruunao)