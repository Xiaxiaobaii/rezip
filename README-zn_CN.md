# ReZIP

Rezip是一个用于将各种效率较低的压缩格式（算法）转换为统一的7z (ZSTD)的简单工具

使用rust编写.

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