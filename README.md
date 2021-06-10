# AES-Rust

一个用 Rust 实现的非常简易的 128-bit AES-ECB 库及命令行程序

> 密码学作业，顺便当作自己学习 Rust 的第一个小练习
>
> 第一次用 Rust 写完整的程序，代码风格极差（
>
> 后续可能考虑添加其它模式和其它长度的密钥🕊️

aes 库包含加解密等操作函数

主程序为命令行程序，用于读写文件及调用库函数进行加解密

其中用到以下 Rust 库

- [structopt](https://docs.rs/structopt/0.3.21/structopt/) 用于解析命令行参数
- [hex](https://docs.rs/hex/0.4.3/hex/) 用于解析输入的 hex-format 密钥