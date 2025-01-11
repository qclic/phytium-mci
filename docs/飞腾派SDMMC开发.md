# 前言

现在对于飞腾派的 `arceos`的开发是非常的成熟的,但是对于一个小白是非常麻烦的.

> 这里一定要注意可以跟踪驱动的开发进程,也就是看 `commit`的过程

```shell
 * 1.0   zhugengyu  2021/12/2    init
 * 1.1   zhugengyu  2022/6/6     modify according to tech manual.
 * 1.2   zhugengyu  2022/7/15    adopt to e2000
 * 1.3   zhugengyu  2022/11/23   fix multi-block rw issues
 * 2.0   zhugengyu  2023/9/16    rename as sdif, support SD 3.0 and rework clock timing
 * 2.1   zhugengyu  2023/10/23   add sdio interrupt handler
```

> 这里应该选取所有文件都在的 `V1.1`版本.这个版本在总的 `V0.2.0`里边

[V0.2.0版本](https://gitee.com/phytium_embedded/phytium-standalone-sdk/tree/v0.2.0/)

# 资源

[飞腾派数据手册V1.0版本](https://github.com/arceos-usb/arceos_experiment/blob/usb-camera-base/doc/resources/%E9%A3%9E%E8%85%BE%E6%B4%BE%E6%95%B0%E6%8D%AE%E6%89%8B%E5%86%8CV1.0%E7%89%88%E6%9C%AC.pdf)

萤火工场·CEK8903飞腾派软件开发手册-V1.01(待上传)

- [ ] todo

[飞腾派软件编程手册V1.0](https://github.com/arceos-usb/arceos_experiment/blob/usb-camera-base/doc/resources/%E9%A3%9E%E8%85%BE%E6%B4%BE%E8%BD%AF%E4%BB%B6%E7%BC%96%E7%A8%8B%E6%89%8B%E5%86%8CV1.0.pdf)

[Phytium-Standalone-SDK:](https://gitee.com/phytium_embedded/phytium-standalone-sdk)

[提交 · Phytium嵌入式软件](https://gitee.com/phytium_embedded/phytium-standalone-sdk/commits/master)

# blogs

[Linux MMC 驱动子系统详解 - Buttering&#39;s Blog](https://buttering.github.io/EasyBlog/2023/02/07/Linux%20MMC%20%E9%A9%B1%E5%8A%A8%E5%AD%90%E7%B3%BB%E7%BB%9F%E8%AF%A6%E8%A7%A3/)

[【MMC子系统】一、MMC_SD_SDIO介绍 | Donge Blog](https://uniondong.github.io/docs/linux/linux_mmc_subsystem/mmc%E5%AD%90%E7%B3%BB%E7%BB%9F%E4%B8%80mmc_sd_sdio%E4%BB%8B%E7%BB%8D/)

[MMC/SD/SDIO介绍](http://www.wowotech.net/basic_tech/mmc_sd_sdio_intro.html)

# 已有工作解读

## 预备知识

**萤火工场·CEK8903飞腾派软件开发手册-V1.01/6.高手进阶/高手进阶**

### 交叉编译环境搭建

- [ ] todo

### `make disk_img`指令生成

引用了 `dosfstools`工具,这个工具是Ubuntu预装的.

### 烧录工具

1. SD卡
2. 读卡器
3. CH340-USB-TTL

### phytium飞腾派SDK

[fsdif.md · Phytium嵌入式软件](https://gitee.com/phytium_embedded/phytium-standalone-sdk/blob/master/doc/reference/driver/fsdif.md)

# 飞腾派烧录和运行ArceOS

使用仓库:https://github.com/qclic/arceos/tree/bsp/phytium_pi

注意`clone`下来的是`qcl_dev`分支,本次实验也是使用的这个分支.

如果要切换到`bsp/phytium_pi`分支,执行`git checkout bsp/phytium`.

原理:

1. 使用本地`ostool`,可直接访问USB和网卡,可以使用`uboot.rs`脚本执行串口/网卡的选择,并且建立FTP服务器.
2. 使用`Docker`环境,编译`ArceOS`的镜像.
3. 使用`ubuntu`或者`debian`的`UBOOT`,使用FTP服务器把编译好的`.bin`文件烧录进`phytium`.
4. 需要把串口接好,网口无论是通过路由器作为交换机还是直接网线连接物理网卡,都可以.

因此,我们需要构建`windows`上的`rust`开发环境,并且需要搭建一个用于编译的`Docker`环境.

## 环境搭建

### 本地环境搭建

#### 安装rust(windows)

参考[rust官网](https://www.rust-lang.org/zh-CN/tools/install)一键下载安装

#### 安装ostool

```shell
cargo install ostool
```

构建过程中出现问题:
```shell
error[E0463]: can't find crate for `std`
  |
  = note: the `x86_64-pc-windows-msvc` target may not be installed
  = help: consider downloading the target with `rustup target add x86_64-pc-windows-msvc`
  = help: consider building the standard library from source with `cargo build -Zbuild-std`
```

> `rustup component add rust-std`安上标准库即可
> 因为是在`x86`上构建的环境`rustup target add aarch64-unknown-none`添加这个编译目标项

#### Docker环境搭建

##### 安装Docker

[官网安装](https://www.docker.com/)

##### 预先pull一个镜像

> 使用的国内能够`pull`下来的镜像

```shell
docker pull doublezonline.cloud/library/rust:slim
```

##### ~~构建Docker环境(可以不构建)~~

修改`Dockerfile`内容,把第一行换成镜像站下载的名称:
```docker
FROM doublezonline.cloud/library/rust:slim
... ...
```

```shell
docker build -t arceos -f Dockerfile .
```

##### ~~~~进入docker环境(可以不进入)~~

> 原本的指令`$(pwd)`不能在`powershell`里被识别,换成`${PWD}`

```shell
docker run -it -v ${PWD}:/arceos -w /arceos arceos bash
```

##### ~~~~尝试运行ArceOS by QEMU(可以不测试)~~

```shell
make A=examples/helloworld ARCH=aarch64 run
```

### ~~WSL2环境搭建~~

参考[arceos/README.md at bsp/phytium_pi · qclic/arceos](https://github.com/qclic/arceos/blob/bsp/phytium_pi/README.md)搭建好基础环境

参考[连接 USB 设备 | Microsoft Learn](https://learn.microsoft.com/zh-cn/windows/wsl/connect-usb#install-the-usbipd-win-project)分享USB设备到WSL2

#### 打开CH340的驱动

**每次启动电脑都要弄**

```shell
sudo modprobe ch341
sudo modprobe usbserial
sudo modprobe cp210x
sudo modprobe ftdi_sio
```

#### 变更权限

```shell
sudo chmod a+rw /dev/ttyUSB0
sudo setcap cap_net_bind_service=+eip $(which ostool)
```

> 记得重启终端

#### 存在插不稳问题

- [ ] 需要编写脚本

### 运行测试代码

> 这里只在本地环境测试了

```shell
ostool run uboot
```

会生成一个`.project.toml`,如果需要调整就可以删除它重新生成.

这里由于当前版本的`ostool`不支持选择`platform`.

```toml
[compile]
target = "aarch64-unknown-none-softfloat"

[compile.custom]
shell = [["docker build -t arceos -f Dockerfile ."], ["docker run --rm -it -v .:/arceos", '-v "D:\03_workspace\phytium_arceos\arceos\target\docker_cache\git:/usr/local/cargo/git"', '-v "D:\03_workspace\phytium_arceos\arceos\target\docker_cache\registry:/usr/local/cargo/registry"', "-w /arceos arceos", "make A=examples/helloworld ARCH=aarch64 PLATFORM=aarch64-phytium-pi"]]
elf = "examples/helloworld/helloworld_aarch64-phytium-pi.elf"

[qemu]
machine = "virt"
cpu = "cortex-a53"
graphic = false
args = "-smp 2"

[uboot]
... ...
```

只需保存你的`[uboot]`的部分,其余部分:
1. 调整了`make`语句,增加了PLATFORM=aarch64-phytium-pi
2. 调整`elf`文件名
3. 如果需要更改`APP`名称,需要修改`make`里的`A=`和`elf`名称

## 解析OStools代码

- [ ] todo 了解关于串口的打开的代码

# 可能需要

`gdb`调试

# LOG

- 1.9 更换仓库名称为phytium-mci,仓库地址[qclic/phytium-mci: sd mmc driver](https://github.com/qclic/phytium-mci)
-
