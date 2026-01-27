# ReZIP

Convert all compressed archives using various algorithms into 7z archives with Zstandard (Zstd) compression.

language use rust.

```
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