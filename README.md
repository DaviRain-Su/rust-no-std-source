# rust-no-std-source

About Rust no std sources

- std和no_std的区别
- Rust中的no_std的一些使用方法
- 验证一个库是否支持no_std的验证方式
- 具体的写一个no_std的库
- 一些no_std和std可以使用primitive类型的仓库





## 1. std 和 no_std 的区别



## 2. Rust中no_std的一些使用方法



## 3. 验证一个库是否支持no_std的验证方式

`
cargo check --target wasm32-unknown-unknown
`

但是wasm环境不一定就是no_std，或者别的编译目标也可以，也就是裸露的编译目标环境不带有任何系统的环境。

参考文档: [使用Rust编写操作系统（一）：独立式可执行程序](https://zhuanlan.zhihu.com/p/53064186)


## 4. 具体的写一个no_std的库

### 创建一个no_std库的第一种方式

1. [创建一个仓库](https://github.com/DaviRain-Su/rust-no-std-source/commit/cd90f28855cfe794c235976bb58c1c5ecb8c7fa9)

```
cargo new --lib create-no-std-lib-1
```

2. [使用#![no_std]将这个仓库中的函数能支持在no_std和std下使用](https://github.com/DaviRain-Su/rust-no-std-source/commit/d3c05920865a44ab7cbaf82a72f21c7b6b8beeb0)

```
rust-no-std-source/create-no-std-lib-1  🍣 main 📝 ×2🦀 v1.55.0-nightly 🐏 7GiB/8GiB | 9GiB/9GiB
🕙 11:28:02 ❯ cargo test
   Compiling create-no-std-lib-1 v0.1.0 (/Users/davirain/davirain/rust-no-std-source/create-no-std-lib-1)
    Finished test [unoptimized + debuginfo] target(s) in 0.51s
     Running unittests (/Users/davirain/davirain/rust-no-std-source/target/debug/deps/create_no_std_lib_1-01d268f91a23f421)

running 2 tests
test tests::it_works ... ok
test tests::test_sum ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests create-no-std-lib-1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


rust-no-std-source/create-no-std-lib-1  🍣 main 📝 ×2🦀 v1.55.0-nightly 🐏 7GiB/8GiB | 9GiB/9GiB
🕙 11:28:07 ❯ cargo test --no-default-features
    Finished test [unoptimized + debuginfo] target(s) in 0.02s
     Running unittests (/Users/davirain/davirain/rust-no-std-source/target/debug/deps/create_no_std_lib_1-01d268f91a23f421)

running 2 tests
test tests::it_works ... ok
test tests::test_sum ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests create-no-std-lib-1

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

使一些不能在no_std环境下运行的函数也能在no_std下支持



[开始添加一个函数编译报错commit 1](https://github.com/DaviRain-Su/rust-no-std-source/commit/8bcd0b909ee116d3dc9c6464c2548e1c008d672e)

```
    Checking create-no-std-lib-1 v0.1.0 (/Users/davirain/davirain/rust-no-std-source/create-no-std-lib-1)
error: cannot find macro `format` in this scope
  --> create-no-std-lib-1/src/lib.rs:10:5
   |
10 |     format!("hello")
   |     ^^^^^^

error[E0412]: cannot find type `String` in this scope
 --> create-no-std-lib-1/src/lib.rs:9:30
  |
9 | pub fn get_hello_string() -> String {
  |                              ^^^^^^ not found in this scope

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0412`.
error: could not compile `create-no-std-lib-1`

To learn more, run the command again with --verbose.
```

[修复错误commit 2]()
```
rust-no-std-source  🍣 main 📝 ×2🦀 v1.55.0-nightly 🐏 7GiB/8GiB | 9GiB/9GiB
🕙 11:36:06 ✖  cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```



## 5. 一些no_std和std可以使用的primite类型仓库

- [sp-std](https://github.com/paritytech/substrate/tree/master/primitives/std)

