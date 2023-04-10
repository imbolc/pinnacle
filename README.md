[![License](https://img.shields.io/crates/l/pinnacle.svg)](https://choosealicense.com/licenses/mit/)
[![Crates.io](https://img.shields.io/crates/v/pinnacle.svg)](https://crates.io/crates/pinnacle)
[![Docs.rs](https://docs.rs/pinnacle/badge.svg)](https://docs.rs/pinnacle)

<!-- cargo-sync-readme start -->

# pinnacle

Rust Wrapper for [Pinnacle Sports API][api]

> **Note**
> Not all of the API is currently wrapped, but it should be relatively easy to add missing
> endpoints. All you need to do is implement the corresponding
> [request](`traits::PinnacleApiRequest`) and probably a [response](`responses`).
> Don't hesitate to make a PR if you do.

Here are all the currently wrapped [`requests`].

## Usage

```rust,no_run
use pinnacle::prelude::*;

#[tokio::main]
async fn main() {
   let client = PinnacleClient::new("pinnacle_user", "pinnacle_password");
   let req = GetStraightOdds {
       sport_id: 29,
       ..Default::default()
   };
   let resp = client.get(&req).await.unwrap();
   dbg!(resp);
}
```

[api]: https://pinnacleapi.github.io/

<!-- cargo-sync-readme end -->

## Contributing

We appreciate all kinds of contributions, thank you!


### Note on README

Most of the readme is automatically copied from the crate documentation by [cargo-sync-readme][].
This way the readme is always in sync with the docs and examples are tested.

So if you find a part of the readme you'd like to change between `<!-- cargo-sync-readme start -->`
and `<!-- cargo-sync-readme end -->` markers, don't edit `README.md` directly, but rather change
the documentation on top of `src/lib.rs` and then synchronize the readme with:
```bash
cargo sync-readme
```
(make sure the cargo command is installed):
```bash
cargo install cargo-sync-readme
```

If you have [rusty-hook] installed the changes will apply automatically on commit.


## License

This project is licensed under the [MIT license](LICENSE).

[cargo-sync-readme]: https://github.com/phaazon/cargo-sync-readme
[rusty-hook]: https://github.com/swellaby/rusty-hook
