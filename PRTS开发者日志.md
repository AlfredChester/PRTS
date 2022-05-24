<div>
    <center>
        <h1 style="font:50px">PRTS开发者日志</h1>
    </center>
</div>

## Day0

    **准备了汇编以及OS的运行环境**

---

## Day1

### 1) 什么是二进制编辑器(十六进制编辑器)

> <mark>一种直接编写二进制指令,使CPU直接执行的软件</mark>
> 
> <mark>一般使用十六进制编写,再转换为16进制</mark>

## 2) .img文件的虚拟机运行环境

> 使用Nask进行编译,runbox运行
> 
> 使用complie.bat 自动编译运行

### 3)  初次体验汇编程序 (指令汇总在[Nasm.md](./Nasm.md)中)

#### 1.DB指令

        DB,Define Byte

        往  <mark>文件中</mark>  直接写入一个字节(或多个字节)

        语法为:DB %d....

```nasm
DB 0xff,0xfd
```

        这句语句的目的,就是往文件里分别依次写入0xff 0xfd

        附录:文件大小单位以及转换

| 名称:    | 位    (Bit) | 字节(Byte)   | 千字节(Kb)       | 兆字节(Mb)     | 千兆字节(GB)    | 千GB  (TB)   |
| ------ | ---------- | ---------- | ------------- | ----------- | ----------- | ----------- |
| **转换** | **/**      | **=8*Bit** | **=1024Byte** | **=1024Kb** | **=1024Mb** | **=1024GB** |

<center>表1.1 数据单位的转换表</center>

#### 2.RESB指令

        RESB,Reserve Byte

        从第一个未被填充的字节开始 ,一直填充0x00,重复指定的次数或者到指定的  <mark>文件中地址</mark>

        语法为 RESB %d / RESB %d-$

```nasm
RESB 100
```

        往文件中写入100个0x00

```nasm
RESB 0x1fe-$
```

        一直写入0x00,直到`0x1fe`的文件地址

#### 3.注释

        以`;` 开始 同C/C++中的`//`

        

#### 4.DW指令

        DW,Define Word 

        写入2个字节的数据,16位数据

        语法为: DW ...

```nasm
DW 0xff
```

等于:

```nasm
DB 0x00,0xff
```

#### 5.DD指令

        DD,Define Doubleword

        写入4个字节的数据,32位数据

        语法为: DD ...

```nasm
DD 0xff
```

等于:

```nasm
DW 0x00,0xff
```

#### 6.DQ指令

        DQ,Define Quote

        写入4个字节的数据,32位数据

        语法为: DQ ...

```nasm
DQ 0xff
```

等于:

```nasm
DD 0x00,0xff
```

### 4) 专业名词解释

#### 1.TAB=4

        将TAB键的长度设为4个空格键,令源程序更加可读

#### 2.FAT12格式
