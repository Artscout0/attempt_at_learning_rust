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

Sometimes, you might not want to run all the tests. You can do that, by writing 
```bash
cargo test fn_name
```
this will run all the tests whose names contain fn_name.

## Ignoring tests

If some code takes like an hour to test, say you're training an LLM, you can use the \#\[ignored\] tag before it. Now, if you want to test it, you can just run 
```bash
cargo test -- --ignored
```
or 
```bash
cargo test -- --include-ignored
```