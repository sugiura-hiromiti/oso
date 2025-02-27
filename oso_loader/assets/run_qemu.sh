# NOTE: 前回実行時のファイルを削除
rm -rf $PWD/assets/mnt $PWD/assets/disk.img

# NOTE: raw disk imageを作成
qemu-img create -f raw $PWD/assets/disk.img 64m

# NOTE: diskイメージをフォーマット
mkfs.fat -n 'OSO' -s 2 -f 2 -R 32 -F 32 $PWD/assets/disk.img

# NOTE: マウントポイントを作成してイメージをマウント
mkdir -p $PWD/assets/mnt
MOUNTED_DISK=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount $PWD/assets/disk.img)
mount -t msdos $MOUNTED_DISK $PWD/assets/mnt

#  NOTE: efiアプリケーションをマウントポイントに移動
mkdir -p $PWD/assets/mnt/efi/boot
if [ "$1" = "-x86_64" ]; then
	echo 'on x86_64 mode'
	cp $2 $PWD/assets/mnt/efi/boot/bootx64.efi
elif [ "$1" = "-aarch64" ]; then
	echo 'on aarch64 mode'
	cp $2 $PWD/assets/mnt/efi/boot/bootaa64.efi
fi

# NOTE: unmount disk
hdiutil detach $MOUNTED_DISK

if [ "$1" = "-x86_64" ]; then
	# on x86
	qemu-system-x86_64 \
		-drive if=pflash,file=$PWD/assets/OVMF_CODE.fd,format=raw,readonly=on \
		-drive if=pflash,file=$PWD/assets/OVMF_VARS.fd,format=raw \
		-hda $PWD/assets/disk.img \
		-monitor stdio
elif [ "$1" = "-aarch64" ]; then
	# on aarch64
	qemu-system-aarch64 \
		-M virt \
		-machine virtualization=true \
		-machine virt,gic-version=3 \
		-cpu max \
		-smp 2 \
		-m 4096 \
		-drive if=pflash,format=raw,file=$PWD/assets/disk.img,readonly=on \
		-drive format=raw,file=$PWD/assets/varstore.img \
		-monitor stdio \
		# -cpu max,pauth-impdef=on \
fi
