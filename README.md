# rust axum tutorial
https://www.youtube.com/watch?v=QCktBeTkOjA&list=PLrmY5pVcnuE-_CP7XZ_44HN-mDrLQV4nS&index=7

# add dependencies
- cargo add axum
- cargo add tokio
- cargo install cargo-modules

# use 
```
cargo-modules structure --lib
```

# create new branch
git checkout -b feature
git push origin 
git push --set-upstream origin feature

# create pull request

# Container Result
use std::{num::ParseIntError, result::Result};
use std::option::Option;

- unwrap extracts a value in the container if present.
- .ok converts between Result and Option.
- .map converts to the same container but with a different type.
- ? operator is used to propagate the absent of a value or a success up the stack.