// 添加cfg_attr宏可以使得这个库可以在std的环境和no_std的环境之间转换
// 这句话表达的意思是当不是std的时候启用的是no_std
// 在cargo.toml中的配置使用这个crate的时候，将default-features 设置为false,就是将默认的std环境关闭启动的是no_std
#![cfg_attr(not(feature = "std"), no_std)]


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
