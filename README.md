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

## TODO

1. Update user-agent list via CRON github actions.
2. Add option to setup native runtime CRON to update list in real time.
3. Add option to output UA value to a language of choice file for importing.
