# Rust Hexagonal Architecture

## Create a new project

```shell
cargo new rust-hexagonal-architecture
```

## Create our first use case

```
src
├── pokemon
│   ├── create_pokemon.rs
│   └── mod.rs
└── main.rs
```

- `src/pokemon/create_pokemon.rs`

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_return_the_pokemon_number_otherwise() {
        let number = 25;
        let req = Request {
            number,
            name: String::from("Pikachu"),
            types: vec![String::from("Electric")],
        };

        let res = execute(req);

        assert_eq!(res, number);
    }
}
```

- `src/pokemon/mod.rs`

```rust
mod create_pokemon;
```

- `src/main.rs`

```rust
mod pokemon;

fn main() {}
```

### Thank you

- https://github.com/alexislozano/pokedex/tree/master
- https://alexis-lozano.com/hexagonal-architecture-in-rust-1/