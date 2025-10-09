# intro


# build and run

```
cargo build
cargo run
```

# test

test all

```
cargo test -- --test-threads=1
```

test apptest

```
cargo test --test app_tests -- --test-threads=1
cargo test --test reg_codes_tests -- --test-threads=1
cargo test --test resources_tests -- --test-threads=1
cargo test --test role_tests -- --test-threads=1
cargo test --test user_tests -- --test-threads=1
```

