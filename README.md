# memo

## bootloader

- システムパーティションはアーキテクチャによって名前が異なる

| arch         | file name        |
| :----------- | :--------------- |
| intel 32bit  | bootia32.efi     |
| 86_64        | bootx64.efi      |
| aarch32      | bootarm.efi      |
| aarch64      | bootaa64.efi     |
| riscv 32bit  | bootriscv32.efi  |
| riscv 64bit  | bootriscv64.efi  |
| riscv 128bit | bootriscv128.efi |

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

- uefi-rsクレートのAPI移行

について [詳しく書かれたドキュメント](https://github.com/rust-osdev/uefi-rs/blob/13c1c2be2b17edd73a9565d89431a9266273f8a8/docs/funcs_migration.md?plain=1#L50)

- `rm`しようとするとresource busyとなる場合

1. マウントされたままかを確認

```sh
diskutil list
# or
mount
```

2. マウントされていた場合は解除する

```sh
umount /path/to/mount_point
# or
diskutil eject /path/to/mount_point
```

- rustにおける[T]型の`into_iter`の挙動

どうやら以下のコードにおける`item`の値は予想の逆順になっているっぽい

```rust
// this program outputs
//
// lol
// omg
//
// not
//
// omg
// lol
for item in ["omg", "lol"] {
	println!("{item}");
}
```
