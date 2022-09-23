# sigop
A CLI tool to optimize your Solidity function signatures. I wanted to create this after seeing
[transmissions11](https://github.com/transmissions11)'s comment about this optimization.

[Inspired by the great work of emn178](https://github.com/emn178/solidity-optimize-name).

## 🧪 How does it work ?

The optimizer takes a function signature such as `myFunction(address)` and tries to combine it with
a suffix generated from a dictionary.

For each combination, the 4-bytes function selector is computed and verified : if it contains a
specified number of zeros at the beginning, the optimization has been found.

## 🚀 Getting started !

### ⚙️ Installation

Installing from cargo:
```shell
cargo install sigop
```

Or building locally from source:
```shell
make build-release
```

### 🏁 Quickstart

```shell
./target/release/sigop -s "myFunction(address)"
```

Which should print:
```shell
[2022-09-23T04:06:03Z INFO  sigop::optimizer] Found this optimization: myFunction_6mI(address)
```

Using `cast`, we can see the optimized function selector:
```shell
$ cast sig "myFunction_6mI(address)"
0x00001926
```

### ✏️ Custom parameters

You can specify custom parameters used by the optimizer:
1. `length`: The maximum size of the suffix following the original function name.
2. `target`: The number of zero-bytes you want to have at the beginning of the optimized function
   selector.

Example:
```shell
$ sigop -s "myFunction(address)" --length=4 --target=3
[2022-09-23T04:06:26Z INFO  sigop::optimizer] Found this optimization: myFunction_LYq3(address)

$ cast sig "myFunction_LYq3(address)"
0x0000006d
```

### Results

Using Remix, we can track the gas cost of calling these functions:
```shell
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.14;

contract Test {
    // Execution cost : 22132
    function myFunction(address a) public pure returns (address) {
        return a;
    }

    // Execution cost : 22074
    function myFunction_LYq3(address a) public pure returns (address) {
        return a;
    }
}
```

## 🤖 Author

Made with ❤️ by 🤖 [Luca Georges François](https://github.com/0xpanoramix) 🤖