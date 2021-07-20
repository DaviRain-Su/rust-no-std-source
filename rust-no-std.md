# Rust no-std 的常见问题


## 什么要写这个？

大多数Rustaceans（包括我）自从学会了如何写`println!("hello world")`之后，
每天都在使用`std crate`。
但它仍然是Rust的一个非常重要的功能：
将我们的代码部署到裸机环境。
没有操作系统的支持，我们就不能使用`std crate`，
这通常会让人感到害怕，所以我写这篇文章来澄清对`Rust no-std`的大部分误解。

## 什么是Rust no-std？

在`std Rust`中，它是默认的学习版本。
我们可以做很多操作来与机器和互联网进行交互，比如向控制台打印信息，
从文件中读取，以及打开URL。
所有这些功能都是由底层执行环境提供的：
我们的操作系统。我们的操作系统提供了几个系统调用来支持IO、网络、文件系统和进程；
Rust通过这些系统调用将这些功能委托给操作系统。

你可以翻阅std的模块，并尝试识别哪些模块依赖于操作系统。
当然，如果我们没有一个提供底层实现的操作系统，我们就不能使用这些功能。
一个叫做`no-std`的功能就是用于这些裸机环境。
在`no-std Rust`中，我们只能使用不依赖操作系统的核心特性。
看看`core crate`；`core crate`是`std crate`的一个子集；
你可以在核心中找到许多熟悉的模块，实现内存操作、算术或常用的类型结构。

std和no-std之间的差别很小:

- 在no-std中，你不能使用std crate，但是，你可以从核心中导入大部分模块。
- 你不能使用与堆相关的模块（box, collections, string, etc.），因为默认的Rust内存分配器依赖于操作系统的系统调用来增加堆内存；除非你实现自己的全局分配器。
- 如果你写一个bin crate，你必须实现一些lang项。

不要被这些陌生的术语吓到；
要理解这些，你需要知道一些rustc的概念，比如lang item或global allocator，
这些概念在std环境中被我们隐藏起来了。

## 什么是lang item？

简而言之：Rustc被设计成可插拔的；
Rustc允许用户通过lang项来定制语言功能，而不是在编译器中内置所有操作。

长话短说：语言项文件

大部分的语言项都是在核心箱中定义的；但是，有些是在std箱中定义的。
例如，eh_personality是由失败机制使用的。
如果你在写一个无std的bin crate，你需要实现这些lang项以使编译器工作；
但如果你在写一个lib crate，你可以假设bin crate定义了这些lang项，
所以你不需要这样做。

`lang`项的功能是不稳定的，这意味着我们只能在夜间的Rust中定义lang项。
Rust团队通过编译器属性公开了一些lang项；
它允许我们在稳定的Rust中定义它们，
例如:`#[panic_handler]`定义了`panic_impl lang`项，而`#[alloc_error_handle]`定义了`oom lang`项。

一个建议是，在你试图从头开始实现它们之前，你应该始终寻找一个运行时支持板块。Rust嵌入式工作小组是一个很好的开始。他们提供了为不同的嵌入式环境定义lang项的板条；通过使用这些板条，你可以忘记lang项，获得更好的生活。

## 什么是alloc crate？什么是全局分配器？

`alloc crate`包含与堆相关的模块；
`alloc`中的模块使用全局分配器来分配内存。
`std crate`定义了一个默认的全局分配器，它取决于操作系统；
当堆内存耗尽时，`std`全局分配器会调用操作系统的系统调用来增加内存。
所以在`no-std`环境下，我们需要定义我们的全局分配器；
我们可以使用`#[global_allocator]`属性来定义它。
通常情况下，我们使用一个固定的内存范围作为我们的堆；
当堆用尽时，我们不会调用`brk`或`mmap`
（Linux系统调用，向操作系统索取更多的内存），
而是引发一个内存不足的错误。

有许多全局分配器的实现；
例如，最简单的是以链表的形式实现的；
这里是我使用好友分配器算法写的一个，
它可以保证在不同情况下的稳定响应时间。

通过定义全局分配器，我们可以在`no-std`程序中使用`alloc crate`。
`alloc`包含了非常频繁使用的模块，如`string`、`box`、`collections` etc。
`core`和`alloc crate`几乎涵盖了我在`std`中最经常使用的模块。

## 如何编写no-std lib crate

通过在`lib.rs`的顶部添加`#![no_std]`，
我们告诉`rustc`在`no-std Rust`下编译整个crate；
如果我们试图从std导入或使用依赖std的crate，
编译器会引发错误。通常，我们使用另一个编译条件 
`#![cfg_attr(not(test), no_std)]`来告诉 `rustc `
只在测试标志被禁用时才编译到`no-std Rust`，
这样我们就可以在测试中使用`std`，就像 std Rust 那样。

如果我们需要使用 `alloc crate`，
我们需要在 `lib.rs` 的 `extern crate alloc` 中再添加一行；
因为 `alloc` 是一个内置的 `crate`，所以 `rustc` 会自动为我们链接它。

```rust
//! lib.rs
#![cfg_attr(not(test), no_std)]

/// Add this line if you need to use alloc modules
extern crate alloc;
```

## 如何在我的crate中同时支持std和no-std环境？
习惯性的方法是使用[货物特性](https://doc.rust-lang.org/cargo/reference/features.html#features)

我们在Cargo.toml中添加一个`std`特性，并将其作为默认设置。

```toml
# Cargo.toml
[features]
default = ["std"]
std = []
```

然后在lib.rs中我们使用std特性作为编译条件。

```rust
//! lib.rs
#![cfg_attr(not(feature = "std"), no_std)]

/// different implementations under std or no-std

#[cfg_attr(feature = "std")]
fn a () { // std implementation }

#[cfg_attr(not(feature = "std"))]
fn a () { // no-std implementation }
```

因为我们将`std`定义为默认特性，所以我们的测试仍然在`std Rust`中进行编译。

我们也可以控制依赖性来启用`std`特性。

```toml
# Cargo.toml
[features]
default = ["std"]
std = ["crate-a/std", "crate-b/std"]

[dependencies]
crate-a = { version = "0.1", default-features = false }
crate-b = { version = "0.1", default-features = false }
```
