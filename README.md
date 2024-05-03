# ua_generator

Generate random real User-Agents to use as spoofs.

## How to use

Example of generating a randomized user agent updated weekly.

```
cargo add ua_generator --git https://github.com/spider-rs/ua_generator.git
```

```rust
use ua_generator::ua::spoof_ua;

fn main() {
    let ua = spoof_ua();

    println!("{}", ua);
    // Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36
};
```

The static files are checked and generated from the `build.rs` file to prevent extra need for API calls since they are limited and require authentication.

If you plan on using this building locally you need a valid API key from the [Spider Cloud API](https://spider.cloud/api-keys) and set the value to the env var `APILAYER_KEY`. You can run `BUILD_ENABLED=1 APILAYER_KEY=$APILAYER_KEY cargo build` to generate the static files. You can also change the API endpoint using the env variable `API_URL`.