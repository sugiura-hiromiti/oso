# memo

## bootloader

- システムパーティションはアーキテクチャによって名前が異なる

| arch         | file name         |
| :----------- | :---------------- |
| intel 32bit  | bootia32.efi      |
| 86_64        | bootix64.efi      |
| aarch32      | bootiarm.efi      |
| aarch64      | bootiaa64.efi     |
| riscv 32bit  | bootiriscv32.efi  |
| riscv 64bit  | bootiriscv64.efi  |
| riscv 128bit | bootiriscv128.efi |

- 外部記憶装置を特定のフォルダにマウントすると通常のファイルシステムのように扱う事ができる(中身をlsコマンドなどを使って見る事ができる)

macでのマウント例

```sh
# mount
MOUNTED_DISK=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount ./disk.img)
mount -t msdos $MOUNTED_DISK mnt

# unmount
hdiutil detach $MOUNTED_DISK
```

- ABI

rustcを使っている限りはABIの規約を守るのはコンパイラの役割
アセンブリを使う場合はプログラマがABIの規約を守る必要がある

x86_64向けのABIではSystem V AMD64が一般的
aarch64向けを作る場合はABIの違いを考慮する必要がありそう
