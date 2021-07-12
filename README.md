# rust-no-std-source

About Rust no std sources

- std和no_std的区别
- Rust中的no_std的一些使用方法
- 验证一个库是否支持no_std的验证方式
- 具体的写一个no_std的库
- 一些no_std和std可以使用primitive类型的仓库





## 1. std 和 no_std 的区别

> 2.1.3 核心库
>
> Rust语言的语法由核心库和标准库共同提供。
> 其中Rust核心库是标准库的基础。核心库中定义的是Rust语言的核心，不依赖于操作系统和网络等相关的库，甚至不知道堆分配，也不提供并发和I/O
>
> 可以通过在模块顶部引入＃！[no_std]来使用核心库。核心库和标准库的功能有一些重复，包括如下部分：
> - 基础的trait，如Copy、Debug、Display、Option等。
> - 基本原始类型，如bool、char、i8/u8、i16/u16、i32/u32、i64/u64、isize/usize、f32/f64、str、array、slice、tuple、pointer等。
> - 常用功能型数据类型，满足常见的功能性需求，如String、Vec、HashMap、Rc、Arc、Box等。
> - 常用的宏定义，如println！、assert！、panic！、vec！等。
> 做嵌入式应用开发的时候，核心库是必需的。
>
> 2.1.4 标准库
>
> Rust标准库提供应用程序开发所需要的基础和跨平台支持。标准库包含的内容大概如下：
> - 与核心库一样的基本trait、原始数据类型、功能型数据类型和常用宏等，以及与核心库几乎完全一致的API。
> - 并发、I/O和运行时。例如线程模块、用于消息传递的通道类型、Sync trait等并发模块，文件、TCP、UDP、管道、套接字等常见I/O。
> - 平台抽象。os模块提供了许多与操作环境交互的基本功能，包括程序参数、环境变量和目录导航；路径模块封装了处理文件路径的平台特定规则。
> - 底层操作接口，比如 std：：mem、std：：ptr、std：：intrinsics 等，操作内存、指针、调用编译器固有函数。
> - 可选和错误处理类型Option和Result，以及各种迭代器等。


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

[修复错误commit 2](https://github.com/DaviRain-Su/rust-no-std-source/commit/ae94f9cf147b7ce37632cb4e9c36e20c5135b3ad)
```
rust-no-std-source  🍣 main 📝 ×2🦀 v1.55.0-nightly 🐏 7GiB/8GiB | 9GiB/9GiB
🕙 11:36:06 ✖  cargo check
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
```



## 5. 一些no_std和std可以使用的primite类型仓库

- [sp-std](https://github.com/paritytech/substrate/tree/master/primitives/std)


## 引用

[rust编程之道核心库和标准库的介绍](https://weread.qq.com/web/reader/0303203071848774030b9d6k9bf32f301f9bf31c7ff0a60)

[rust embeded book](https://docs.rust-embedded.org/book/intro/no-std.html)
