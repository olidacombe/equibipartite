# equibipartite

Partition a list of integers into two equal-sum sublists.

## Example

```rust
use equibipartite::get_equi_partition;

let collection = vec![1, 4, 7, 35, 2, 1, 18, 6];
let partition = get_equi_partition(&collection);
println!("{:#?}", partition);

let collection = vec![1, 2, 3, 4, 5, 6];
assert!(get_equi_partition(&collection).is_none());
```

Current version: 0.1.0

License: MIT OR Apache-2.0
