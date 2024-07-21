# 作业三：使用rust编写一个简单的内核模块并运行   
## 创建 rust\_helloworld.rs 文件   
进入到 `Linux` 目录下的 `samples/rust` 文件夹，创建一个新的文件 `rust\_helloworld.rs`，并添加以下内容：   
```
// SPDX-License-Identifier: GPL-2.0
//! Rust minimal sample.
      
use kernel::prelude::*;
      
module! {
  type: RustHelloWorld,
  name: "rust_helloworld",
  author: "whocare",
  description: "hello world module in rust",
  license: "GPL",
}
      
struct RustHelloWorld {}
      
impl kernel::Module for RustHelloWorld {
  fn init(_name: &'static CStr, _module: &'static ThisModule) -> Result<Self> {
      pr_info!("Hello World from Rust module\n");
      Ok(RustHelloWorld {})
  }
}

```
## 修改模块配置文件   
### 修改 Kconfig   
打开 `samples/rust/Kconfig` 并添加以下内容，以便在 `menuconfig` 中配置 `rust\_helloworld` 模块：   
```
config SAMPLE_RUST_HELLOWORLD
    tristate "Print Helloworld in Rust"
    help
      This is a simple example of a Rust kernel module.
```
### 修改 Makefile   
打开 `samples/rust/Makefile` 并添加以下内容，以便编译 `rust\_helloworld.rs`：   
```
obj-$(CONFIG_SAMPLE_RUST_HELLOWORLD) += rust_helloworld.o
```
## 配置和编译内核   
在 `linux` 目录下运行以下指令   
```
make LLVM=1 menuconfig
```
更改该模块的配置，使之编译成模块：   
```
Kernel hacking
  ---> Sample Kernel code
      ---> Rust samples
              ---> <M>Print Helloworld in Rust (NEW)
```
### 重新编译内核   
```
make LLVM=1 -j$(nproc)
```
`samples/rust` 路径下生成了 `rust\_helloworld.ko` 的文件，将文件复制到 `src\_e1000/rootfs` 目录下   
## 运行虚拟环境   
```
./build_image.sh
```
安装模块：
   
```
insmod rust_helloworld.ko
```
![image.png](files\image.png)    
