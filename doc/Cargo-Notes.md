# Testing

## Only run integration tests
`cargo test --test '*'`

### Run an individual integration test file
`cargo test --test <file name>`

## Only run library unit tests
`cargo test --lib`

## To remove cached libraries (that may be causing !#[features]) compilation
`cargo clean`
