# 作业二：对Linux内核进行一些配置   
# 具体步骤   
## 编译内核模块   
进入 `src\_e1000` 目录执行指令：   
```
make LLVM=1 -j$(nproc)

```
## 禁用默认的e1000网卡驱动   
进入 Linux 内核文件夹，打开内核配置：   
```
make menuconfig

```
导航到 `Device Drivers > Network device support > Ethernet driver support > Intel devices`，找到 `Intel(R) PRO/1000 Gigabit Ethernet support` 并禁用它。退出并保存配置。   
## 确认模块版本与内核版本一致   
打开**内核 **Makefile 文件，**内核**版本为：`VERSION` + . + `PATCHLEVEL` + . + `SUBLEVEL` + . + `EXTRAVERSION` = 6.1.0-rc1   
```
  GNU nano 7.2                                            Makefile                                                      # SPDX-License-Identifier: GPL-2.0
VERSION = 6
PATCHLEVEL = 1
SUBLEVEL = 0
EXTRAVERSION = -rc1
NAME = Hurr durr I'ma ninja sloth
...
```
 进入 `src\_e1000` 目录使用以下代码查看**模块**版本：   
```
modinfo r4l_e1000_demo.ko

```
## 重新编译内核   
进入 Linux 内核文件夹：   
```
make LLVM=1 -j$(nproc)
```
## 运行脚本文件   
```
./build_image.sh

```
进入 qemu 环境，加载驱动：   
```
insmod r4l_e1000_demo.ko
```
使用以下命令验证模块是否正确加载：   
```
lsmod | grep r4l_e1000_demo

```
配置联网：   
```
ip link set eth0 up
ip addr add broadcast 10.0.2.255 dev eth0
ip addr add 10.0.2.15/255.255.255.0 dev eth0 
ip route add default via 10.0.2.1
ping 10.0.2.2

```
# 实现原理   
## 如何将代码编译为内核模块   
在 `Kbuild` 文件中，通过 obj-m 指定编译为内核模块   
```
# SPDX-License-Identifier: GPL-2.0
obj-m := r4l_e1000_demo.o

```
## out-of-tree module 如何与内核代码产生联系   
**Out-of-tree** 模块是指那些不在 Linux 内核源代码树中的模块，但可以独立于内核源代码进行编译。它们与内核代码产生联系的方式如下：   
```
KDIR ?= ../linux

default:
	$(MAKE) -C $(KDIR) M=$$PWD

```
在模块的 `Makefile` 中，使用 `KDIR` 变量指定 Linux 内核源代码的路径。通过以下语句，将当前模块目录（ `$$PWD`）作为模块的构建目录传递给内核的 `make` 系统。当执行 `make LLVM=1` 命令时， `-C $(KDIR)` 选项会切换到指定的内核源代码目录，然后 `M=$$PWD` 选项告诉内核构建系统，当前模块的源代码位于 `PWD` 目录（即当前目录）。内核构建系统会使用该信息来编译模块。   
![屏幕截图 2024-07-17 173408.png](files\ping-mu-jie-tu-2024-07-17-173408.png)    
