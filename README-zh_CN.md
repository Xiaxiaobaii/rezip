# ReZIP

[English](README.md)

将采用多种算法的压缩归档统一转换为使用 Zstandard（Zstd）压缩的 7z 归档。

**支持的扩展名**
```
Zip (.zip)
RoshalArchive (.rar)
SevenZip (.7z)
```

**测试文件**
```
PortableNetworkGraphics (.png)
JointPhotographicExpertsGroup (.jpeg)
```

语言：Rust。

```
Usage: rezip [OPTIONS] <SELECT_DIR> <OUTPUT_DIR>

Arguments:
  <SELECT_DIR>  
  <OUTPUT_DIR>  

Options:
  -l, --max-depth <MAX_DEPTH>  [default: 1]
  -h, --help                   Print help

/temp> ls
1.zip (Deflate)
2.rar (rar)
3.7z (LZMA)
4.zip (ZSTD)

/temp> rezip /temp /temp/temp
/temp> ls
1.zip
2.rar
3.7z
4.zip
temp

/temp>cd temp
/temp/temp>ls
1.7z (ZSTD)
2.7z (ZSTD)
3.7z (ZSTD)
4.zip (ZSTD)
```
