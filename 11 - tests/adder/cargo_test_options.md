# Cargo Test arguments

## Threads
If you have a ton of test and you really don't want to test all of them on a single thread, you can use --test-threads=N

Syntax:
```bash
cargo test -- --test-threads=3
```

Obviously, this might have some issues, the ususal ones with reading and writing files for example.

## Output
If your functions shows  any output, using for example println!(), you can use --show-output

Syntax:
```bash
cargo test  -- --show-output
```

## Only some tests

Sometimes, you might not want to run all the tests.
