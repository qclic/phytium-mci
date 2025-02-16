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

# Soc型号


这个包括丝印和一些其它的地方都不提及.

> 根据我的查阅应该是E2000Q


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

注意 `clone`下来的是 `qcl_dev`分支,本次实验也是使用的这个分支.

如果要切换到 `bsp/phytium_pi`分支,执行 `git checkout bsp/phytium`.

原理:

1. 使用本地 `ostool`,可直接访问USB和网卡,可以使用 `uboot.rs`脚本执行串口/网卡的选择,并且建立FTP服务器.
2. 使用 `Docker`环境,编译 `ArceOS`的镜像.
3. 使用 `ubuntu`或者 `debian`的 `UBOOT`,使用FTP服务器把编译好的 `.bin`文件烧录进 `phytium`.
4. 需要把串口接好,网口无论是通过路由器作为交换机还是直接网线连接物理网卡,都可以.

因此,我们需要构建 `windows`上的 `rust`开发环境,并且需要搭建一个用于编译的 `Docker`环境.

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
> 因为是在 `x86`上构建的环境 `rustup target add aarch64-unknown-none`添加这个编译目标项

#### Docker环境搭建

##### 安装Docker

[官网安装](https://www.docker.com/)

##### 预先pull一个镜像

> 使用的国内能够 `pull`下来的镜像

```shell
docker pull doublezonline.cloud/library/rust:slim
```

##### ~~构建Docker环境(可以不构建)~~

修改 `Dockerfile`内容,把第一行换成镜像站下载的名称:

```docker
FROM doublezonline.cloud/library/rust:slim
... ...
```

```shell
docker build -t arceos -f Dockerfile .
```

##### ~~进入docker环境(可以不进入)~~

> 原本的指令 `$(pwd)`不能在 `powershell`里被识别,换成 `${PWD}`

```shell
docker run -it -v ${PWD}:/arceos -w /arceos arceos bash
```

##### ~~尝试运行ArceOS by QEMU(可以不测试)~~

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

会生成一个 `.project.toml`,如果需要调整就可以删除它重新生成.

这里由于当前版本的 `ostool`不支持选择 `platform`.

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

只需保存你的 `[uboot]`的部分,其余部分:

1. 调整了 `make`语句,增加了PLATFORM=aarch64-phytium-pi
2. 调整 `elf`文件名
3. 如果需要更改 `APP`名称,需要修改 `make`里的 `A=`和 `elf`名称

## 解析OStools代码

- [ ] todo 了解关于串口的打开的代码

# 裸机驱动开发

仓库地址:https://github.com/qclic/phytium-mci

## 尝试运行

类似于 `ArceOS-Phytium`的同样运行原理,用 `ostool`编译/烧录/运行相关的程序代码.

> 一定要记得 `reset`.

生成的 `.bare-test.toml`如下所示:

```toml
serial = "COM7"
baud_rate = 115200
net = "以太网"
dtb_file = "frimware/phytium.dtb"
```

## 裸机驱动

参照:[fsdif.md · Phytium嵌入式软件](https://gitee.com/phytium_embedded/phytium-standalone-sdk/blob/master/doc/reference/driver/fsdif.md)

### 了解接口和寄存器

寄存器都存放在 `drivers\mmc\fsdif\fsdif_hw.h`里,考虑把它整体先转为 `rust`,放在 `src\constants.rs`.并且注意在 `src\lib.rs`里声明这个模块.

### 接口移植

**FSdifCfgInitialize**:初始化SDIF控制器初始化

> FSdifCfgInitialize 调用了 FSdifReset
>
> 1. 说明有的接口是做了但是不开放的,要考虑
> 2. 要首先实现这个 FSdifReset

**FSdifDeInitialize**:解除SDIF控制器初始化

### SD卡的空间设置

> 这里的第一反应是去看Win32DiskImager,但是没有得到具体答案.

这里直接登录烧录进去的 `ubuntu`系统.

> 这里注意 `飞腾派V3版本ubuntu镜像 241212\更新日志.txt`里有很多我们需要的信息
> 用户名:user
> 密码:user

查看分区:

```shell
user@Phytium-Pi:~$ lsblk
NAME        MAJ:MIN RM   SIZE RO TYPE MOUNTPOINT
loop0         7:0    0     4K  1 loop /snap/bare/5
loop1         7:1    0  68.9M  1 loop /snap/core22/1720
loop2         7:2    0  64.7M  1 loop /snap/cups/1069
loop3         7:3    0 483.3M  1 loop /snap/gnome-42-2204/178
loop4         7:4    0  91.7M  1 loop /snap/gtk-common-themes/1535
loop5         7:5    0  38.6M  1 loop /snap/snapd/23259
loop6         7:6    0  38.7M  1 loop /snap/snapd/23546
mmcblk0     179:0    0 116.5G  0 disk
└─mmcblk0p1 179:1    0 116.5G  0 part /
```

可以看到我们插入的128G的SD卡还有116.5G,是被设置为了 `disk`.
这时候我们就需要它的地址.

查看起止地址:`sudo fdisk -l /dev/mmcblk0`

```shell
user@Phytium-Pi:~$ sudo fdisk -l /dev/mmcblk0
Disk /dev/mmcblk0：116.52 GiB，125103505408 字节，244342784 个扇区
单元：扇区 / 1 * 512 = 512 字节
扇区大小(逻辑/物理)：512 字节 / 512 字节
I/O 大小(最小/最佳)：512 字节 / 512 字节
磁盘标签类型：dos
磁盘标识符：0x0001a0bd

设备           启动   起点      末尾      扇区   大小 Id 类型
/dev/mmcblk0p1      131072 244342783 244211712 116.5G 83 Linux
```

按byte读取sd卡内容:

```shell
sudo dd if=/dev/mmcblk0p1 of=output.bin bs=1 count=1024
sudo dd if=/dev/mmcblk0p1 of=output.bin bs=512 count=2 skip=999
sudo dd if=/dev/mmcblk0p1 of=output.bin bs=512 count=3 skip=998
hexdump -C output.bin
sudo dd if=/dev/mmcblk0p1 of=output.bin bs=512 count=1 skip=100 | hexdump -C output.bin
sudo dd if=/dev/mmcblk0p1 of=output.bin bs=512 count=1 skip=99 | hexdump -C output.bin
```

# 检查寄存器

## reset之后的寄存器

```shell
-------------------------------
cntrl: 0x2000010
0010 0000 0000 0000 0000 0001 0000
/* RW 全局中断使能配置, 1 使能 */
/* RW 使用内部DMA */
启用了DMA
启动了全局中断使能.
-------------------------------
pwren: 0x1
0001
// RW 卡供电开关, 0：关；1：开
启动了卡供电.
-------------------------------
clkdiv: 0x30204
0011 0000 0010 0000 0100
DIVDER: 0000 0100 :4
DRV: 0000 0010 :2
SAMPLE: 0000 0011 :3
-------------------------------
clkena: 0x1
0001
/* RW 0：Clock disabled；1：Clock enabled */
启动时钟.
-------------------------------
tmout: 0xffffffff
11111111111111111111111111111111
读卡超时（以卡时钟为单位）: 0xffffff
响应超时（以卡时钟为单位）: 0xff
-------------------------------
ctype: 0x10000
0001 0000 0000 0000 0000
/* 1: 8-bit mode */
!!! 这里应该修正为/* 0: 1-bit mode */
-------------------------------
blksz: 0x200
512 Byte一个块.
-------------------------------
blkcnt: 0x1000
4096个待传输数据(复位值?和手册上说的不一样)
-------------------------------
intmask: 0x1
0001
/* RW Card detect (CD) */
只开启卡检测中断.
-------------------------------
cmdarg: 0x0
0
没用
-------------------------------
cmd: 0x200000
001000000000000000000000
使能 HOLD Register
-------------------------------
resp0: 0xb00
reps1: 0xa4177f80
resp2: 0xdb590003
resp3: 0x400e0032
-------------------------------
maskints: 0x0
没任何中断被屏蔽?没搞懂这个寄存器的作用
-------------------------------
rawints: 0x0
没触发任何中断
-------------------------------
status: 0x6106
0110 0001 0000 0110
/* RO DATA[3] 卡在位检测，1：在位 */
/* RO, 达到 FIFO_TX 标记 */
/* RO, FIFO empty */
0110 0 响应索引号: 12
-------------------------------
fifoth: 0x20070100
0010 0000 0000 0111 0000 0001 0000 0000
0001 0000 0000 : TX_WMARK 100
0000 0000 0111 : RX_WMARK 7
0010 : DMA 2 多次传输的突发大小
-------------------------------
carddet: 0x0
卡在位
-------------------------------
wrtprt: 0x0
无写保护
-------------------------------
cksts: 0x1
/* CIU 时钟 ready */
-------------------------------
trans_cardcnt: 0x1000
Device 接口模块到 Device 传输的字节数为 4096
-------------------------------
trans_fifocnt: 0x1000
MEM&FIFO 之间传输的字节数 4096
-------------------------------
debnce: 0xffffff
去抖时钟数，参考值 5-25 ms 但是这里是满的,是复位默认值,或者意味着不去抖?
-------------------------------
uid: 0x59595959
用户 ID: 复位值
-------------------------------
vid: 0x6488280a
控制器版本: 复位值
-------------------------------
hwconf: 0x10001
无描述,无操作
-------------------------------
uhsreg: 0x0
使用3.3V的电压
-------------------------------
cardreset: 0x0
复位:0
-------------------------------
busmode: 0x280
0010 1000 0000
idma使能
010：8 transfers
-------------------------------
descaddrl: 0xf9c39c00
不是复位值,不知道什么情况
-------------------------------
descaddrh: 0x0
复位值
-------------------------------
dmacstatus: 0x0
未发生任何DMA中断
-------------------------------
dmacinten: 0x0
没有使能任何DMA中断
-------------------------------
curdescaddrl: 0xf9c39c00
不是复位值,不知道什么情况
-------------------------------
curdescaddrh: 0x0
复位值
-------------------------------
card_thrctl: 0x800001
1000 0000 0000 0000 0000 0001
/* RW 读卡threshold使能 */
FIFO深度是 Depth8
-------------------------------
clock_src: 0x502
0101 0000 0010
CIU时钟使能
0101 CLK_DIV: 5
-------------------------------
emmcddr: 0x0

-------------------------------
enableshift: 0x0

-------------------------------
```

应该是忽略了SD_INIT后续的函数,要细查

# 遇到的问题

- [ ] 一直存在FsdifCtrl的CONTROLLER_RESET一直为1的情况

# note

> OSA: Operating System Abstraction 操作系统抽象层

/* RW Start-bit error (SBE) */ 报了这个interrupt.

# 可能需要

`gdb`调试

# LOG

- 1.9 更换仓库名称为phytium-mci,仓库地址[qclic/phytium-mci: sd mmc driver](https://github.com/qclic/phytium-mci)
-
