# 项目小实验   
# 环境配置   
## 添加环境变量   
```
export R4L_EXP=/usr/rust/cicv-r4l-3-KaidRommel/r4l_experiment/
```
## 创建initramfs镜像   
创建目录：   
```
mkdir $R4L_EXP/initramfs
cd $R4L_EXP/initramfs
```
```
mkdir -p {bin,dev,etc,lib,lib64,mnt,proc,root,sbin,sys,tmp}

```
将 `/tmp` 目录的权限设置为 `1777` ，确保所有用户都可以在目录内进行读取、写入和执行操作，同时设置“粘滞位”：   
```
chmod 1777 tmp

```
从主机复制必要的设备文件到当前目录下的 `dev` 子目录：   
```
sudo cp -a /dev/{null,console,tty,ttyS0} dev/

```
## 拷贝 busybox 文件到 initramf/bin 目录下   
```
cd $R4L_EXP/initramfs

```
将BusyBox二进制文件从指定路径复制到当前目录下的 `bin` 目录中：   
```
cp /usr/rust/cicv-r4l-3-KaidRommel/busybox-1.36.1/busybox  ./bin/
```
赋予BusyBox可执行权限：   
```
chmod +x bin/busybox

```
### 安装 busybox    
```
bin/busybox --install bin
bin/busybox --install sbin

```
## 编写init脚本   
```
cd $R4L_EXP/initramfs

```
使用 `cat` 命令和重定向符号 `<<EOF` 来创建一个多行的 `init` 脚本文件:   
```
cat << EOF > init
#!/bin/busybox sh

# 挂载 /proc 和 /sys 文件系统。
mount -t proc none /proc
mount -t sysfs none /sys

# 启动实际服务。

# 启用网卡
ip link set eth0 up
ip addr add 10.0.2.15/24 dev eth0
ip link set lo up

# 等待网卡准备就绪
sleep 0.5

# 使用 -l 选项将新的 shell 设置为登录 shell
# 只有登录 shell 会读取 /etc/profile
setsid sh -c 'exec sh -l </dev/ttyS0 >/dev/ttyS0 2>&1'

EOF

```
赋予 `init` 脚本可执行权限：   
```
chmod +x init

```
## 其他脚本   
创建并配置 `etc/hosts` 文件，用于主机名解析：   
```
cat << EOF > etc/hosts
127.0.0.1    localhost
10.0.2.2     host_machine
EOF
```
创建并配置 `etc/profile` 文件，定义常用别名：   
```
cat << EOF > etc/profile
alias ll='ls -l'
EOF
```
创建并配置 `etc/passwd` 文件，定义系统用户信息：   
```
cat << EOF > etc/passwd
root:x:0:0:root:/root:/bin/bash
EOF
```
创建并配置 `etc/group` 文件，定义系统组信息：   
```
cat << EOF > etc/group
root:x:0:
EOF
```
### 构建 initramfs 镜像   
```
cd $R4L_EXP/initramfs

```
将 `$R4L\_EXP/initramfs` 目录中的所有文件和子目录打包成一个 `cpio` 归档文件，并使用 `gzip` 进行压缩，最终生成 `initramfs.cpio.gz` 文件：   
```
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz

```
## 通过boot.sh脚本启动   
### 创建 `boot.sh` 脚本   
```
cd $R4L_EXP

```
```
cat << EOF > boot.sh
#!/bin/sh
kernel_image="../linux/arch/x86/boot/bzImage"

qemu-system-x86_64 \
-kernel $kernel_image \
-append "console=ttyS0" \
-initrd ./initramfs.cpio.gz \
-nographic
EOF
```
赋予 boot.sh 脚本可执行权限：   
```
chmod +x boot.sh

```
### 执行 boot 脚本   
```
./boot.sh  # Press <C-A> x to terminate QEMU.

```
## 支持 NFS   
安装NFS服务器：   
```
sudo apt-get install nfs-kernel-server

```
配置NFS共享：   
```
sudo bash -c "echo '/usr/rust/cicv-r4l-3-KaidRommel/r4l_experiment/driver 127.0.0.1(insecure,rw,sync,no_root_squash)/driver 127.0.0.1(insecure,rw,sync,no_root_squash)' >> /etc/exports"

```
创建目录：   
```
sudo mkdir -p /run/sendsigs.omit.d
```
重启NFS服务：   
```
sudo /etc/init.d/rpcbind restart
sudo /etc/init.d/nfs-kernel-server restart

```
### 在 init 脚本中添加自动挂载   
在 `sleep 0.5` 后添加自动挂载的命令，使用绝对路径：   
```
mount -t nfs -o nolock host_machine:/usr/rust/cicv-r4l-3-KaidRommel/r4l_experiment/driver /mnt

```
重新构建 `initramfs`：   
```
cd $R4L_EXP/initramfs
```
```
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
```
## 配置 telnet server   
```
cd $R4L_EXP/initramfs
```
创建目录：   
```
mkdir dev/pts
```
创建一个名为 `ptmx` 的设备节点文件，并设置其权限和类型：   
```
mknod -m 666 dev/ptmx c 5 2
```
- `mknod` 是一个用于创建特殊文件的命令。
`-m 666` 指定文件的权限为 `666`（即所有用户均可读写）。
`dev/ptmx` 是要创建的设备节点文件路径。
`c` 表示创建的是字符设备（而不是块设备）。
`5 2` 是主设备号和次设备号的组合，用于标识 `ptmx` 设备。主设备号 `5` 和次设备号 `2` 是系统内核预定义的，用于表示伪终端主设备。   
   
### 修改 init 脚本   
在NFS设置后面加入：   
```
# 挂载 devpts
mount -t devpts devpts /dev/pts

# 启动 telnet 服务器
telnetd -l /bin/sh
```
重新构建 `initramfs`：   
```
find . -print0 | cpio --null -ov --format=newc | gzip -9 > ../initramfs.cpio.gz
```
### 修改 boot.sh   
在boot.sh中加入一下qemu启动参数：   
```
-netdev user,id=host_net,hostfwd=tcp::7023-:23 -device e1000,mac=52:54:00:12:34:50,netdev=host_net
```
### 安装 telnet 客户端   
```
sudo apt-get install telnet
```
### 在本机通过telnet server连接qemu控制台   
```
telnet localhost 7023
```
# 重构 Linux C 代码   
## 编写 makefile    
```
ifneq ($(KERNELRELEASE),)

# In kbuild context
module-objs := completion.o	
obj-m := completion.o

CFLAGS_hello_world.o := -DDEBUG
else
KDIR := ../../../linux
PWD := $(shell pwd)

all:
	$(MAKE) LLVM=1 -C $(KDIR)  M=$(PWD) modules

.PHONY: clean
clean:
	rm -f *.ko *.o .*.cmd .*.o.d *.mod *.mod.o *.mod.c *.symvers *.markers *.unsigned *.order *~
endif
```
# 测试   
### 将代码编译成内核模块：   
```
cd $R4L_EXP/driver/003_completion_rust
```
```
make LLVM=1 -j$(nproc)
```
### 通过 `telnet` 链接虚拟环境：   
```
telnet localhost 7023
```
### 加载模块   
```
cd /mnt/003_completion_rust

```
执行脚本：   
```
rmmod completion.ko
```
```
./load_module.sh
```
### 测试结果   
```
cat /dev/completion
```
![image.png](files\image_j.png)    
```
echo "Hello" > /dev/completion
```
![image.png](files\image_q.png)    
   
