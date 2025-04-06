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

- ESP

EFI System Partitionの略
通常はブートローダなどが置かれる

---

# uefi-rs aarch64

```sh
qemu-system-aarch64 \
	-nodefaults \
	-device virtio-rng-pci \
	-boot menu=on,splash-time=0 \
	-fw_cfg name=opt/org.tianocore/X-Cpuhp-Bugcheck-Override,string=yes \
	-machine virt \
	-cpu cortex-a72 \
	-device virtio-gpu-pci \
	-drive if=pflash,format=raw,readonly=on,file=target/ovmf/aarch64/code.fd \
	-drive if=pflash,format=raw,readonly=off,file=/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/ovmf_vars \
	-drive format=raw,file=fat:rw:target/aarch64-unknown-uefi/debug/esp \
	-drive format=raw,file=/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/test_disk.fat.img \
	-serial pipe:/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/serial \
	-qmp pipe:/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/qemu-monitor \
	-nic user,model=e1000,net=192.168.17.0/24,tftp=uefi-test-runner/tftp/,bootfile=fake-boot-file
```

# uefi-rs x86

```sh
qemu-system-aarch64 \
	-nodefaults \
	-device virtio-rng-pci \
	-boot menu=on,splash-time=0 \
	-fw_cfg name=opt/org.tianocore/X-Cpuhp-Bugcheck-Override,string=yes \
	-machine virt -cpu cortex-a72 \
	-device virtio-gpu-pci \
	-drive if=pflash,format=raw,readonly=on,file=target/ovmf/aarch64/code.fd \
	-drive if=pflash,format=raw,readonly=off,file=/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/ovmf_vars \
	-drive format=raw,file=fat:rw:target/aarch64-unknown-uefi/debug/esp \
	-drive format=raw,file=/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/test_disk.fat.img \
	-serial pipe:/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/serial \
	-qmp pipe:/var/folders/hy/cv8qx4mn6vs6csg4jh5mml140000gn/T/.tmpFLDhad/qemu-monitor \
	-nic user,model=e1000,net=192.168.17.0/24,tftp=uefi-test-runner/tftp/,bootfile=fake-boot-file
```

# oso aarch64

```sh
qemu-system-aarch64 \
	-machine virt \
	-cpu cortex-a72 \
	-device virtio-gpu-pci \
	-drive if=pflash,format=raw,readonly=on,file=/tmp/aarch64/code.fd \
	-drive if=pflash,format=raw,readonly=off,file=/Users/a/Downloads/QwQ/oso/target/xtask/ovmf_vars \
	-drive file=/Users/a/Downloads/QwQ/oso/target/xtask/disk.img,format=raw,if=none,id=hd0 \
	-device virtio-blk-device,drive=hd0 \
	-boot menu=on,splash-time=0
```

# oso x86

```sh
qemu-system-x86_64 \
	-machine q35 \
	-smp 4 \
	-vga std \
	-drive if=pflash,format=raw,readonly=on,file=/tmp/x64/code.fd \
	-drive if=pflash,format=raw,readonly=off,file=/Users/a/Downloads/QwQ/oso/target/xtask/ovmf_vars \
	-drive file=/Users/a/Downloads/QwQ/oso/target/xtask/disk.img,format=raw,if=none,id=hd0 \
	-device virtio-blk-pci,drive=hd0 \
	-boot menu=on,splash-time=0
```
