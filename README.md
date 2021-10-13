# binary_tree

Program that builds a binary tree in memory, serialize's the binary tree to disk and deserialize from disk and re-constructs in memory.

## build and run with cargo

```
cd binary_tree/
cargo build
cargo run
```

## dependencies
- serde
- serde_json

## binary_tree_serialized.json

```
{
  "NonEmpty": {
    "data": "Federal",
    "left_sub_tree": "Empty",
    "right_sub_tree": {
      "NonEmpty": {
        "data": "State",
        "left_sub_tree": "Empty",
        "right_sub_tree": {
          "NonEmpty": {
            "data": "city",
            "left_sub_tree": "Empty",
            "right_sub_tree": "Empty"
          }
        }
      }
    }
  }
}
```

# dependencies
```Cargo.toml```






- serde
- serde_json


## Output
