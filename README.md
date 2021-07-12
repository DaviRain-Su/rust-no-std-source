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


## 5. 一些no_std和std可以使用的primite类型仓库

- [sp-std](https://github.com/paritytech/substrate/tree/master/primitives/std)

