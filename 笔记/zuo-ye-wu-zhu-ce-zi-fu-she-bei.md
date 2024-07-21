# 作业五：注册字符设备   
# 编写 write/read 函数   
```
fn write(this: &Self,_file: &file::File,reader: &mut impl kernel::io_buffer::IoBufferReader,offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let mut vec = this.inner.lock();
        // 计算要写入的数据长度，取 reader 中数据的长度和 vec 中从 offset 开始的剩余空间的最小值
        let len = core::cmp::min(reader.len(), vec.len().saturating_sub(offset));

        // 从 reader 中读取数据，并写入到 vec 中从 offset 开始的位置，长度为 len。如果读取失败，函数会返回错误
        reader.read_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }

    fn read(this: &Self,_file: &file::File,writer: &mut impl kernel::io_buffer::IoBufferWriter,_offset:u64,) -> Result<usize> {
        let offset = offset.try_into()?;
        let vec = this.inner.lock();
        let len = core::cmp::min(writer.len(), vec.len().saturating_sub(offset));
        writer.write_slice(&mut vec[offset..][..len])?;
        Ok(len)
    }
```
# 编译   
```
make menuconfig
```
选中 `Character device` ，按 `Y` 编译进内核：   
```
Kernel hacking
  ---> Sample Kernel code
      ---> Rust samples
              ---> <*>Character device (NEW)

```
编译内核：   
```
make LLVM=1 -j$(nproc)
```
# 测试   
进入虚拟环境：   
```
./build_image.sh
```
![image.png](files\image_x.png)    
