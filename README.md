# ua_generator

Generate random real User-Agents to use as spoofs.

## How to use

Example of generating a randomized user agent.

```rust
use ua_generator::ua::spoof_ua;

fn main() {
    let ua = spoof_ua();

    println!("{}", ua); // Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36
};
```

If you plan on using this library or building directly you need a valid API key from the [API Layer User Agent API](https://apilayer.com/marketplace/user_agent-api) and set the value to the env var `APILAYER_KEY`.

## TODO

1. Update user-agent list via CRON github actions.
2. Add option to setup native runtime CRON to update list in real time.
