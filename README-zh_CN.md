# ReZIP

[English](README.md)

将采用多种算法的压缩归档统一转换为使用 Zstandard（Zstd）压缩的 7z 归档。

计划将项目升级为自动把所有旧格式转换为更前沿的新格式。

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
  <SELECT_DIR>  origin file dir
  <OUTPUT_DIR>  output dir

Options:
  -l, --max-depth <MAX_DEPTH>
          Scan the directory depth of "select_dir" [default: 1]
      --zstd-compress-level <ZSTD_COMPRESS_LEVEL>
          set zstd compress level [default: 16]
  -d, --delete-origin
          delete origin file instead of keep
      --decompress-zstd-zip
          decompress zip (zstf) instead of move
  -h, --help
          Print help

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
