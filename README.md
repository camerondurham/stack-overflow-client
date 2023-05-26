# stack-overflow (a very unofficial Rust client)

A feature incomplete Stack website (as in Stack Overflow) api wrapper for fun and no profit.

## Examples

Create a default client (for the StackOverflow site) and query for featured questions.

```rust
let client = StackClient::new();

let results = client
                .get_featured_questions("docker")
                .await
                .expect("Unable to fetch featured docker questions");

dbg!(&results);
```

Create a client specifically for the [Meta Stack Exchange Site](https://meta.stackexchange.com) and query for featured API questions.

```rust
let client = StackClientBuilder::new()
              .stack_site(StackSite::Meta)
              .version(ApiVersion::V2_3)
              .build();

let results = client
                .get_featured_questions("api")
                .await
                .expect("Unable to fetch featured api questions");

dbg!(&results);
```

## TODO

- [ ] Add more API endpoints
- [ ] Add filter/sorting options to API endpoints
- [ ] Allow [authentication](https://api.stackexchange.com/docs/authentication) to support endpoints and paging beyond page 25
