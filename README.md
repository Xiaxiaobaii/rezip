# ReZIP

[中文](README-zh_CN.md)

Convert compressed archives using various algorithms into 7z archives with Zstandard (Zstd) compression.

**Supported extensions**
```
Zip (.zip)
RoshalArchive (.rar)
SevenZip (.7z)
```

**Test files**
```
PortableNetworkGraphics (.png)
JointPhotographicExpertsGroup (.jpeg)
```

Language: Rust.

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
