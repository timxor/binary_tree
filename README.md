# binary_tree

Program that builds a binary tree in memory, serialize's the binary tree to disk and deserialize from disk and re-constructs in memory.

## build and run with cargo

```
cd binary_tree/
cargo build
cargo run
```



## run unit tests

``` cargo test ```











This generates `binary_tree_serialized.json` from an inline dummy `BinaryTree` struct:

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



`BinaryTree` struct is defined in `/binary_tree/src/main.rs`.


## compile single file

```rustc src/main.rs```
```./src/hello```


## searlize
- https://www.educative.io/m/serialize-deserialize-binary-tree
- https://leetcode.com/problems/serialize-and-deserialize-binary-tree/


## rust unit tests
- cargo test
- https://doc.rust-lang.org/rust-by-example/cargo/test.html
- automated tests: https://doc.rust-lang.org/book/ch11-00-testing.html
- https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html


## json mock data

some mock data json:

```
{
    "champion": "Lee Changho",
    "games": [
        {
            "white": "Honinbo Shusai",
            "black": "Go Seigen",
            "moves": [
                {
                    "time_seconds": 4,
                    "coordinate": [16, 2],
                },
                {
                    "time_seconds": 9,
                    "coordinate": [2, 3],
                },
                "Followed by 246 more moves from the first game",
            ],
            "result": 1.5,
        },
        "Followed by 119 more games from the tournament",
    ]
}
```
