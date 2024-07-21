# 作业一：编译Linux内核   
## 安装依赖：   
安装 `flex` 工具。 `flex` 是一个生成词法分析器（lexical analyzer）的工具，它在编译 Linux 内核时是必需的。   
```
sudo apt-get install flex
```
安装 `bison` 工具。`bison` 是一个生成语法分析器（parser）的工具，它在编译 Linux 内核时是必需的。   
```
sudo apt-get install bison
```
安装 LLD 工具。`ld.lld` 是 LLVM 的链接器，它在使用 LLVM/Clang 编译时是必需的。   
```
sudo apt-get install lld
```
安装  `libelf` 库   
```
sudo apt-get install libelf-dev

```
安装 OpenSSL 库   
```
sudo apt-get install libssl-dev
```
`bc` 是一个基本计算器工具，内核编译过程中需要使用。   
```
sudo apt-get install bc
```
 --- 
**一键安装以上依赖**   
```
sudo apt-get install build-essential flex bison libelf-dev libssl-dev bc
```
## 指定 Rust 版本   
安装指定版本的 Rust 工具链（包括标准库源代码）：   
```
rustup toolchain install 1.62.0 --component rust-src

```
在你的 Linux 内核源代码目录中创建一个名为 `rust-toolchain.toml` 的文件，内容如下：   
```
[toolchain]
channel = "1.62.0"

```
## 进入Linux文件夹，使用如下命令进行编译：   
```
make x86_64_defconfig
```
用来生成一个基于 x86\_64 架构的默认配置文件。这个配置文件包含了适用于大多数 x86\_64 系统的默认选项。执行这个命令后，会在当前目录下生成一个 `.config` 文件。   
 --- 
```
make LLVM=1 menuconfig
```
这一行命令使用 `menuconfig` 进行内核配置，允许用户通过图形界面（基于 ncurses 的终端界面）来配置内核选项。其中 `LLVM=1` 表示使用 LLVM/Clang 作为编译器，而不是默认的 GCC。   
```
#set the following config to yes
General setup
        ---> [*] Rust support

```
启用“Rust support”选项。这将会在内核中启用对 Rust 编程语言的支持。   
 --- 
```
make LLVM=1 -j$(nproc)
```
这行命令用于编译内核， `LLVM=1` 再次指定使用 LLVM/Clang 作为编译器。
`-j$(nproc)` 选项表示使用所有可用的 CPU 核心进行并行编译，以加快编译速度。 `$(nproc)` 会自动替换为当前系统中的 CPU 核心数。   
**在最后你将在Linux文件夹下，得到一个vmlinux的文件，那么就算成功了**   
![屏幕截图 2024-07-17 151625.png](files\ping-mu-jie-tu-2024-07-17-151625.png)    
![屏幕截图 2024-07-17 151723.png](files\ping-mu-jie-tu-2024-07-17-151723.png)    
