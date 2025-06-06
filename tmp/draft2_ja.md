# UEFI環境でaarch64カーネル実行が出来ずに万策尽きかけた話

## はじめに

最近前々から興味のあったOS開発をしています
せっかく自作するんだから、ということでBootloaderも含めてフルスクラッチから作成しており、Pure Rustで書かれたaarch64向けオペレーティングシステムです
Bootloaderから自作しよう！となるとネット上の情報も少なく、またPure RustでのOS開発というのも依然ニッチなジャンルです
さらにそこにaarch64向けというフィルタも加えると更に正しく有益なTipsを見つける事は難しくなります
体感としてもこれは実感していて、ドキュメントが少なくあったとしても結局 ~~クソ長い退屈な~~ 1次情報である仕様書/規格書に辿り着く、
やっと分かりやすそうな情報にたどり着いたと思ったらx86_64向けの内容だったという事がよくあります

この記事では、そんなニッチなジャンルにも同胞がいる事を信じて、"aarch64向けPure RustフルスクラッチOS開発流行れ流行れ"という呪詛を振り撒くと共に、実際に自分が直面した問題の中で一番厄介だった事例を紹介、解決策の解説をしてみます

この記事が対象としている人：

- UEFI環境で動くブートローダを書きたい人
- RustでOS開発をしたい人
- aarch64向けのOS開発をしたい人
- OS開発にQEMUを使ってる人

aarch64向けPure RustフルスクラッチOS開発流行れ流行れ

## ある日：`Display output is not active`

ある日、最小構成のBootloaderが完成しkernelを呼び出そうとしたところ、QEMUウィンドウにDisplay output is not activeと表示されました
この表示を見たのは初めてではなく、仮想環境バックエンドにQEMUを使用している[^UTMのバックエンド]UTMという仮想環境アプリで新しくVMを作成した時などにちょくちょく見かけていました
UTMの場合だと処理に時間がかかっている場合が殆どで暫く待っていると操作できるようになります
なので今回のケースでも待ってれば動き始めるだろ～、と暫くよそ事をしつつ様子を見ていました

この問題は、x86_64では完璧に実行されていたカーネルがaarch64では正しく動作しないという形で現れました。症状は以下の通りです：

1. ブートローダーはカーネルELFファイルを正常にロードした
2. ブートローダーはエントリーポイント（0x40010120）を正確に識別した
3. ブートローダーはUEFIブートサービスを正常に終了した
4. カーネルエントリーポイントへのジャンプは発生した
5. しかし、カーネルコードは期待通りに実行されなかった

特に興味深いのは、この問題がアーキテクチャ固有であったことです。同一のコードがx86_64では動作したがaarch64では失敗したことから、根本的なアーキテクチャの違いが関係していることが示唆されました。

## プロジェクトアーキテクチャ

「oso」プロジェクトは、いくつかの主要コンポーネントを持つモジュラーアーキテクチャに従っています：

```
oso/
├── oso_loader/       # UEFIブートローダー（Rust）
├── oso_kernel/       # カーネル実装（Rust）
├── oso_bridge/       # 共有コードとインターフェース
├── xtask/            # ビルドスクリプトとユーティリティ
└── target/           # ビルド成果物
```

### ブートローダーの実装

ブートローダーの実行フローは以下のように実装されています：

```rust
pub fn boot(&mut self) -> ! {
    // UEFI環境の初期化
    self.init_uefi();

    // ディスクからカーネルELFをロード
    let kernel_elf = self.load_kernel_elf();

    // ELFを解析し実行準備
    let entry_point = kernel_elf.entry_point_address();

    // カーネル用にメモリを準備
    self.prepare_memory_for_kernel();

    // UEFIブートサービスを終了
    self.exit_boot_services(None);

    // カーネルエントリーポイントにジャンプ
    self.jump_to_kernel(entry_point);

    // ここには到達しないはず
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

### カーネルエントリーポイント

カーネルエントリーポイントは`oso_kernel/src/main.rs`で以下のように定義されています：

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // 割り込みを無効化
    unsafe {
        asm!("msr daifset, #2");
    }

    // カーネルコンポーネントの初期化
    init_platform();

    // デバッグ用の待機ループ
    loop {
        unsafe { asm!("wfe"); }
    }
}

#[no_mangle]
#[cfg(target_arch = "x86_64")]
pub extern "C" fn kernel_main() -> ! {
    // x86_64固有の初期化
    unsafe {
        asm!("cli");  // 割り込みを無効化
    }

    // カーネルコンポーネントの初期化
    init_platform();

    // デバッグ用の待機ループ
    loop {
        unsafe { asm!("hlt"); }
    }
}
```

## 体系的な調査

この問題を診断するために、私は潜在的な原因を排除するための体系的なアプローチを採用しました：

### 1. ELFロードとエントリーポイントの検証

まず、カーネルELFファイルが正しくロードされ、エントリーポイントが正確に識別されていることを確認する必要がありました。`readelf`を使用してカーネルバイナリを調査しました：

```bash
$ readelf -h target/oso_kernel.elf
ELF Header:
  Magic:   7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
  Class:                             ELF64
  Data:                              2's complement, little endian
  Version:                           1 (current)
  OS/ABI:                            UNIX - System V
  ABI Version:                       0
  Type:                              EXEC (Executable file)
  Machine:                           AArch64
  Version:                           0x1
  Entry point address:               0x40010120
  Start of program headers:          64 (bytes into file)
  Start of section headers:          2656688 (bytes into file)
  Flags:                             0x0
  Size of this header:               64 (bytes)
  Size of program headers:           56 (bytes)
  Number of program headers:         4
  Size of section headers:           64 (bytes)
  Number of section headers:         14
  Section header string table index: 12
```

また、メモリレイアウトを理解するためにプログラムヘッダーも調査しました：

```bash
$ readelf -l target/oso_kernel.elf
Elf file type is EXEC (Executable file)
Entry point 0x40010120
There are 4 program headers, starting at offset 64

Program Headers:
  Type           Offset             VirtAddr           PhysAddr
                 FileSiz            MemSiz              Flags  Align
  PHDR           0x0000000000000040 0x0000000040000040 0x0000000040000040
                 0x00000000000000e0 0x00000000000000e0  R      0x8
  LOAD           0x0000000000000000 0x0000000040000000 0x0000000040000000
                 0x0000000000000120 0x0000000000000120  R      0x10000
  LOAD           0x0000000000000120 0x0000000040010120 0x0000000040010120
                 0x0000000000000010 0x0000000000000010  R E    0x10000
  GNU_STACK      0x0000000000000000 0x0000000000000000 0x0000000000000000
                 0x0000000000000000 0x0000000000000000  RW     0x0
```

エントリーポイントの実際のコードを確認するために、`.text`セクションを逆アセンブルしました：

```bash
$ readelf --hex-dump=.text target/oso_kernel.elf
Hex dump of section '.text':
  0x40010120 ff4300d1 01000014 5f2003d5 ffffff17 .C......_ ......
```

これは以下のaarch64アセンブリに変換されます：

```asm
0x40010120:  d10043ff  sub      sp, sp, #0x10
0x40010124:  14000001  b        #0x40010128
0x40010128:  d503205f  wfe
0x4001012c:  17ffffff  b        #0x40010128
```

これにより、カーネルコードが正しくロードされ、エントリーポイントアドレスが正確であることが確認できました。

### 2. ブートサービス終了の検証

次に、UEFIブートサービスが適切に終了していることを確認する必要がありました。`exit_boot_services()`の実装は以下のようになっていました：

```rust
pub unsafe fn exit_boot_services(custom_memory_type: Option<MemoryType>) -> MemoryMapOwned {
    let memory_type = custom_memory_type.unwrap_or(MemoryType::LOADER_DATA);

    // メモリマップ用のメモリを割り当て
    let mut buf = MemoryMapBackingMemory::new(memory_type)
        .expect("Failed to allocate memory");

    let mut status = Status::ABORTED;

    // ブートサービス終了を試みる（複数回の試行が必要な場合がある）
    for _ in 0..2 {
        match unsafe { get_memory_map_and_exit_boot_services(buf.as_mut_slice()) } {
            Ok(memory_map) => {
                return MemoryMapOwned::from_initialized_mem(buf, memory_map);
            }
            Err(err) => {
                status = err.status()
            }
        }
    }

    // ここに到達した場合、ブートサービスの終了に失敗
    runtime::reset(ResetType::COLD, status, None);
}
```

ヘルパー関数`get_memory_map_and_exit_boot_services`は以下のように実装されていました：

```rust
unsafe fn get_memory_map_and_exit_boot_services(buf: &mut [u8]) -> Result<MemoryMapMeta> {
    let bt = boot_services_raw_panicking();
    let bt = unsafe { bt.as_ref() };

    // 現在のメモリマップを取得
    let memory_map = get_memory_map(buf)?;

    // メモリマップキーを使用してブートサービスを終了
    unsafe { (bt.exit_boot_services)(image_handle().as_ptr(), memory_map.map_key.0) }
        .to_result_with_val(|| memory_map)
}
```

exit_boot_servicesの呼び出し前にデバッグログを追加し、エラーなく成功していることを確認しました。

### 3. カーネルへのジャンプの検証

カーネルエントリーポイントへのジャンプは関数ポインタを使用して実装されていました：

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // カーネルエントリーポイント用の関数型を定義
    type KernelEntry = extern "C" fn() -> !;

    // エントリーポイントアドレスを関数ポインタに変換
    let kernel_main: KernelEntry = unsafe {
        core::mem::transmute(entry_point)
    };

    // カーネルエントリーポイントを呼び出し
    kernel_main();

    // ここには到達しないはず
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

これをデバッグするために、ジャンプ直前にバイナリブレークポイントを追加しました：

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // デバッグマーカー - 特徴的な命令シーケンス
    unsafe { asm!("nop; nop; nop; nop"); }

    type KernelEntry = extern "C" fn() -> !;
    let kernel_main: KernelEntry = unsafe { core::mem::transmute(entry_point) };

    // エントリーポイントアドレスをデバッグログに記録
    debug!("Jumping to kernel at address: {:#x}", entry_point);

    // カーネルにジャンプ
    kernel_main();

    // フォールバックループ
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

QEMUのモニタ（Ctrl+Alt+2でアクセス）を使用して、実行がこの地点に到達したが、kernel_main()呼び出しから戻ってこなかったことを確認できました。これは、ジャンプ中またはジャンプ直後に問題が発生したことを示唆しています。

### 4. 高度なデバッグ技術

このレベルで作業する場合、従来のデバッグツールは利用できないことがよくあります。私はいくつかの特殊な技術を採用しました：

#### バイナリブレークポイント

メモリダンプで識別できる特徴的な命令シーケンスを戦略的に配置しました：

```rust
// ブートローダー内、カーネルにジャンプする前
unsafe {
    asm!("mov x0, #0xDEAD");
    asm!("mov x1, #0xBEEF");
    asm!("nop; nop; nop; nop");
}

// カーネルエントリーポイント内
unsafe {
    asm!("mov x2, #0xCAFE");
    asm!("mov x3, #0xBABE");
    asm!("nop; nop; nop; nop");
}
```

これらのシーケンスは、メモリダンプを調査する際に認識できる特徴的なパターンを作成します。

#### QEMUモニタコマンド

QEMUのモニタは強力なデバッグ機能を提供します。以下のコマンドを使用しました：

```
info registers        # 現在のレジスタ値を表示
x/10i $pc            # プログラムカウンタから10命令を逆アセンブル
x/20wx 0x40010120    # カーネルエントリーポイントから20ワードを調査
info mem              # メモリマッピング情報を表示
```

#### カスタム待機命令

異なる目的のために異なる待機命令を使用しました：

```rust
#[inline(always)]
pub fn wfi() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "aarch64")]
            asm!("wfi");  // Wait For Interrupt
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}

#[inline(always)]
pub fn wfe() -> ! {
    loop {
        unsafe {
            #[cfg(target_arch = "aarch64")]
            asm!("wfe");  // Wait For Event
            #[cfg(target_arch = "x86_64")]
            asm!("hlt");
        }
    }
}
```

一箇所で`wfi`を使用し、別の場所で`wfe`を使用することで、プログラムカウンタの命令を調査することにより実行がどこで停止しているかを判断できました。

### 5. メモリマッピングの検証

重要な洞察は、QEMUでメモリマッピングを調査したときに得られました。x86_64では、仮想メモリは全アドレス空間でアイデンティティマッピング（仮想アドレス = 物理アドレス）されていました。しかし、aarch64では特定の領域のみがアイデンティティマッピングされていました。

これを確認するために、ブートサービス終了前にUEFIメモリマップをダンプするコードを追加しました：

```rust
fn dump_memory_map(memory_map: &MemoryMap) {
    for descriptor in memory_map.entries() {
        debug!(
            "Type: {:?}, Physical: {:#x}-{:#x}, Virtual: {:#x}, Attributes: {:#x}",
            descriptor.ty,
            descriptor.phys_start,
            descriptor.phys_start + (descriptor.page_count * 4096),
            descriptor.virt_start,
            descriptor.att
        );
    }
}
```

これにより、aarch64ではUEFIが管理するメモリ領域のみのアイデンティティマッピングを保証し、すべてのメモリではないことが明らかになりました。これはx86_64との重要なアーキテクチャの違いです。

## 根本原因：MMU構成の違い

広範な調査の結果、根本原因を特定しました：メモリ管理ユニット（MMU）の構成がx86_64とaarch64のUEFI実装間で大きく異なっていました。

### aarch64 MMUの動作

aarch64では、UEFIはブート中に特定の構成でMMUを有効にします：

1. UEFIが管理するメモリ領域はアイデンティティマッピングされる
2. 他の領域はアイデンティティマッピングされていない可能性がある
3. `exit_boot_services()`の後もMMUは有効のまま

ブートローダーがカーネルエントリーポイント（0x40010120）にジャンプしたとき、このアドレスは仮想アドレスでした。MMUマッピングがこのアドレスの有効な変換を持っていなければ、実行は失敗します。

### x86_64 MMUの動作

対照的に、x86_64では：

1. UEFIは通常、全アドレス空間のアイデンティティマッピングを設定する
2. ページング構造により仮想アドレスが物理アドレスと一致することを保証する
3. このアイデンティティマッピングは`exit_boot_services()`後も持続する

これが、同じコードがx86_64では動作したがaarch64では失敗した理由を説明しています。

## 技術的解決策：MMU管理

解決策には、カーネルにジャンプする前のMMUの明示的な管理が必要でした。以下が実装です：

```rust
pub fn jump_to_kernel(&self, entry_point: usize) -> ! {
    // アーキテクチャ固有の準備
    #[cfg(target_arch = "aarch64")]
    unsafe {
        // データがメモリに書き込まれていることを確認
        asm!("dsb sy");

        // 命令キャッシュをフラッシュ
        asm!("ic iallu");       // すべての命令キャッシュをPoUまで無効化
        asm!("dsb ish");        // キャッシュ操作の完了を確認
        asm!("isb");            // コンテキストを同期

        // SCTLR_EL1を変更してMMUを無効化
        asm!(
            "mrs x0, sctlr_el1",    // 現在のSCTLR_EL1を読み取り
            "bic x0, x0, #1",       // ビット0（M）をクリアしてMMUを無効化
            "msr sctlr_el1, x0",    // SCTLR_EL1に書き戻し
            "isb",                  // 命令同期バリア
            out("x0") _
        );
    }

    // カーネルエントリーポイント用の関数型を定義
    type KernelEntry = extern "C" fn() -> !;

    // エントリーポイントアドレスを関数ポインタに変換
    let kernel_main: KernelEntry = unsafe {
        core::mem::transmute(entry_point)
    };

    // カーネルエントリーポイントにジャンプ
    kernel_main();

    // ここには到達しないはず
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

### SCTLR_EL1について

システム制御レジスタ（SCTLR_EL1）は、MMUを含む基本的なシステム動作を制御します。主要なビットの内訳は以下の通りです：

```
ビット0（M）：MMU有効化
  0 = MMU無効
  1 = MMU有効

ビット2（C）：データキャッシュ有効化
  0 = データキャッシュ無効
  1 = データキャッシュ有効

ビット12（I）：命令キャッシュ有効化
  0 = 命令キャッシュ無効
  1 = 命令キャッシュ有効
```

ビット0をクリアすることで、他の機能はそのままにMMUを無効化します。これにより、仮想アドレスが物理アドレスとして扱われ、実質的にすべてのメモリのアイデンティティマッピングが作成されます。

### 同期バリア

ARMアーキテクチャは、MMU操作に不可欠ないくつかの同期バリアを提供しています：

```
DSB（データ同期バリア）
  - 処理を続行する前にすべてのメモリアクセスが完了することを保証
  - "dsb sy"はすべてのメモリ操作に影響

ISB（命令同期バリア）
  - パイプラインをフラッシュし、すべての前の命令が完了したことを保証
  - システムレジスタを変更した後に不可欠

IC IALLU（統合ポイントまでのすべての命令キャッシュを無効化）
  - すべての命令キャッシュを無効化
  - コードが古い変換でキャッシュされている可能性がある場合に必要
```

これらのバリアは、カーネルにジャンプする前にMMU状態の変更が完全に適用されることを保証します。

## カーネル側の考慮事項

MMUが無効化された状態では、カーネルはこの状態を認識する必要があります。カーネルエントリーポイントは以下のように変更できます：

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // 割り込みを無効化
    unsafe {
        asm!("msr daifset, #2");
    }

    // カーネルページテーブルを設定
    let page_tables = setup_kernel_page_tables();

    // カーネルページテーブルでMMUを有効化
    unsafe {
        // ページテーブルを指定
        asm!("msr ttbr0_el1, {}", in(reg) page_tables.ttbr0_el1);
        asm!("msr ttbr1_el1, {}", in(reg) page_tables.ttbr1_el1);

        // 変換制御を設定
        asm!("msr tcr_el1, {}", in(reg) page_tables.tcr_el1);

        // 変更が見えることを確認
        asm!("isb");

        // SCTLR_EL1のビット0を設定してMMUを有効化
        asm!(
            "mrs x0, sctlr_el1",
            "orr x0, x0, #1",      // ビット0（M）を設定してMMUを有効化
            "msr sctlr_el1, x0",
            "isb",
            out("x0") _
        );
    }

    // カーネル初期化を続行
    init_platform();

    // メインカーネルループに入る
    kernel_main_loop();
}
```

このアプローチにより、カーネルはMMU構成を完全に制御でき、独自の仮想メモリマッピングを設定できます。

## 高度な実装：MMU状態の保存

別のアプローチとして、MMU状態を保存しつつカーネル用の適切なマッピングを確保する方法があります：

```rust
pub fn prepare_kernel_execution(&self, entry_point: usize) -> ! {
    // 現在のMMU構成を取得
    let mut sctlr_el1: u64;
    unsafe {
        asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);
    }

    // MMUが有効かチェック
    let mmu_enabled = (sctlr_el1 & 1) != 0;

    if mmu_enabled {
        // オプション1：MMUを無効化
        unsafe {
            asm!(
                "mrs x0, sctlr_el1",
                "bic x0, x0, #1",
                "msr sctlr_el1, x0",
                "isb",
                out("x0") _
            );
        }

        // オプション2：カーネル用のアイデンティティマッピングを追加
        // これにはページテーブルの操作が必要
        // unsafe {
        //     add_identity_mapping_for_kernel(entry_point);
        // }
    }

    // カーネルにジャンプ
    type KernelEntry = extern "C" fn() -> !;
    let kernel_main: KernelEntry = unsafe { core::mem::transmute(entry_point) };
    kernel_main();

    // ここには到達しないはず
    loop {
        unsafe { asm!("wfi"); }
    }
}
```

## 解決策のテスト

解決策を検証するために、既知のメモリ位置に書き込む簡単なテストカーネルを実装しました：

```rust
#[no_mangle]
#[cfg(target_arch = "aarch64")]
pub extern "C" fn kernel_main() -> ! {
    // メモリにシグネチャを書き込み
    unsafe {
        core::ptr::write_volatile(0x40020000 as *mut u32, 0xCAFEBABE);
    }

    // 待機ループに入る
    loop {
        unsafe { asm!("wfe"); }
    }
}
```

QEMUのモニタを使用して、以下を確認できました：

1. カーネルが正しく実行されていること
2. メモリ書き込みが成功したこと
3. システムが安定していること

## アーキテクチャ固有の考慮事項

この調査は、x86_64とaarch64間の重要なアーキテクチャの違いを浮き彫りにしています：

### aarch64の特徴

1. **MMUの動作**：MMUはUEFIブート中に有効化され、`exit_boot_services()`後も有効のまま
2. **メモリマッピング**：UEFI管理のメモリ領域のみがアイデンティティマッピングされることが保証される
3. **キャッシュコヒーレンシー**：MMU状態を変更する際に明示的なキャッシュ管理が必要なことが多い

### x86_64の特徴

1. **ページング構造**：UEFIで通常アイデンティティマッピングされる多層ページング構造を使用
2. **レガシー互換性**：古いモードとの互換性を維持し、メモリ管理に影響
3. **キャッシュ動作**：ページテーブル変更時にキャッシュ無効化が自動的に処理されることが多い

## より広い意味合いとベストプラクティス

この経験から、クロスアーキテクチャOS開発に関するいくつかの重要な教訓が得られました：

### 1. アーキテクチャを意識した設計

複数のアーキテクチャ向けに開発する場合、基本的な違いを認識することが重要です：

```rust
// アーキテクチャを意識したコードの例
pub fn initialize_platform() {
    #[cfg(target_arch = "aarch64")]
    {
        // aarch64固有の初期化
        init_mmu_aarch64();
        setup_exception_vectors();
    }

    #[cfg(target_arch = "x86_64")]
    {
        // x86_64固有の初期化
        init_gdt();
        init_idt();
    }

    // 共通の初期化
    init_memory_allocator();
}
```

### 2. MMU管理戦略

MMU管理の異なるアプローチにはトレードオフがあります：

| 戦略                                       | 利点                   | 欠点                     |
| ------------------------------------------ | ---------------------- | ------------------------ |
| MMUを無効化                                | シンプル、信頼性が高い | 仮想メモリの利点を失う   |
| すべてのメモリをアイデンティティマッピング | アドレッシングを保持   | ページテーブル操作が必要 |
| カスタムマッピング                         | 完全な制御             | 実装が最も複雑           |

### 3. ベアメタル向けデバッグ技術

OS支援なしで動作するデバッグ技術のツールキットを開発します：

```rust
// ベアメタル開発用のデバッグユーティリティ
pub mod debug {
    // 既知のメモリ位置に書き込む
    pub fn write_debug_signature(signature: u32) {
        unsafe {
            core::ptr::write_volatile(0xDEAD0000 as *mut u32, signature);
        }
    }

    // 特徴的な命令パターンを作成
    pub fn binary_breakpoint(id: u8) {
        unsafe {
            // IDを含むユニークなパターンを作成
            asm!("mov x0, #{}", in(const) id);
            asm!("nop; nop; nop; nop");
        }
    }

    // 検出可能な方法で実行を停止
    pub fn halt_with_code(code: u32) -> ! {
        write_debug_signature(code);
        loop {
            unsafe { asm!("wfe"); }
        }
    }
}
```

### 4. メモリバリアの使用

メモリバリアの適切な使用は信頼性の高い動作に不可欠です：

```rust
// メモリバリアユーティリティ
pub mod barriers {
    // すべてのメモリ書き込みが可視であることを確認
    pub fn data_synchronization_barrier() {
        unsafe { asm!("dsb sy"); }
    }

    // 命令パイプラインをフラッシュ
    pub fn instruction_synchronization_barrier() {
        unsafe { asm!("isb"); }
    }

    // メモリ操作が順序通りに完了することを確認
    pub fn data_memory_barrier() {
        unsafe { asm!("dmb sy"); }
    }

    // システム全体の同期
    pub fn full_system_barrier() {
        data_synchronization_barrier();
        instruction_synchronization_barrier();
    }
}
```

## 高度なトピック：ELFロードとメモリレイアウト

ELF形式とメモリレイアウトを理解することは、カーネルを適切にロードするために重要です。ELFロードプロセスをより詳しく見てみましょう：

```rust
pub fn load_elf(&self, elf_data: &[u8]) -> Result<LoadedElf, ElfError> {
    // ELFヘッダーを解析
    let elf_file = ElfFile::new(elf_data)?;
    let elf_header = elf_file.header;

    // アーキテクチャの互換性を確認
    if !self.is_compatible_architecture(&elf_header) {
        return Err(ElfError::IncompatibleArchitecture);
    }

    // プログラムセグメント用のメモリを割り当て
    let mut loaded_segments = Vec::new();

    for program_header in elf_file.program_headers {
        if program_header.p_type != PT_LOAD {
            continue;
        }

        // メモリ要件を計算
        let virt_addr = program_header.p_vaddr as usize;
        let mem_size = program_header.p_memsz as usize;
        let file_size = program_header.p_filesz as usize;

        // 物理メモリを割り当て
        let phys_addr = self.allocate_physical_memory(mem_size, 0x1000)?;

        // セグメントデータをコピー
        let src_offset = program_header.p_offset as usize;
        let src_data = &elf_data[src_offset..src_offset + file_size];
        unsafe {
            core::ptr::copy_nonoverlapping(
                src_data.as_ptr(),
                phys_addr as *mut u8,
                file_size
            );

            // 残りのメモリをゼロ化（bssセクション）
            if mem_size > file_size {
                core::ptr::write_bytes(
                    (phys_addr + file_size) as *mut u8,
                    0,
                    mem_size - file_size
                );
            }
        }

        // セグメントマッピングを記録
        loaded_segments.push(LoadedSegment {
            virt_addr,
            phys_addr,
            size: mem_size,
            flags: program_header.p_flags,
        });
    }

    Ok(LoadedElf {
        entry_point: elf_header.e_entry as usize,
        segments: loaded_segments,
    })
}
```

### メモリレイアウトの考慮事項

ELFファイルをロードする際には、以下のメモリレイアウトの側面を考慮してください：

1. **仮想アドレスと物理アドレス**：ELFセグメントは仮想アドレスを指定しますが、物理メモリにロードする必要があります
2. **アライメント要件**：メモリ領域は特定のアライメント（通常はページアライン）が必要なことが多い
3. **権限**：異なるセグメントには異なる権限要件（読み取り、書き込み、実行）があります
4. **BSS処理**：BSSセクションはゼロ化する必要がありますが、ELFファイルには格納されていません

## 結論：クロスアーキテクチャ開発の洞察

異なるアーキテクチャで動作するOSを開発するには、ハードウェア動作の微妙な違いを理解する必要があります。この記事で強調されたMMU構成の問題は、遭遇する可能性のあるアーキテクチャ固有の課題の一例に過ぎません。

主な教訓：

1. **アーキテクチャの違いは重要**：ハードウェア動作の根本的な違いにより、あるアーキテクチャで動作するものが別のアーキテクチャでは失敗する可能性があります。

2. **MMU構成は重要**：MMU設定は、特にaarch64では、ブートローダーからカーネルへの移行に大きく影響します。

3. **体系的なデバッグが不可欠**：ベアメタルレベルで作業する場合、体系的なデバッグ技術が最も価値のあるツールです。

4. **メモリバリアが重要**：システム状態、特にMMUを操作する際には、適切な同期が不可欠です。

5. **ドキュメントは少ない**：Rustでのaarch64 UEFIのような特殊な組み合わせでは、複数のソースから情報を集める必要があることがよくあります。

これらの洞察を共有することで、他のOS開発者が同様の落とし穴を避け、クロスアーキテクチャ開発の複雑さをより良く理解するのに役立つことを願っています。

## 参考文献と更なる読み物

これらのトピックについてさらに深く掘り下げたい方のために：

1. [ARM Architecture Reference Manual](https://developer.arm.com/documentation/ddi0487/latest) - ARMアーキテクチャの決定的ガイド
2. [UEFI Specification](https://uefi.org/specifications) - 公式UEFIドキュメント
3. [Rust for Embedded Development](https://docs.rust-embedded.org/) - 組み込みRust開発のリソース
4. [OSDev Wiki](https://wiki.osdev.org/) - OS開発のコミュニティ知識ベース
5. [ELF Specification](https://refspecs.linuxfoundation.org/elf/elf.pdf) - 実行可能リンク形式のドキュメント

## 付録A：完全なMMU管理実装

aarch64用のMMU管理の完全な実装を以下に示します：

```rust
/// aarch64用のMMU構成
pub struct MmuConfig {
    pub ttbr0_el1: u64,  // 変換テーブルベースレジスタ0
    pub ttbr1_el1: u64,  // 変換テーブルベースレジスタ1
    pub tcr_el1: u64,    // 変換制御レジスタ
    pub mair_el1: u64,   // メモリ属性間接レジスタ
}

/// MMUを無効化
pub unsafe fn disable_mmu() {
    // すべてのメモリ操作が完了することを確認
    asm!("dsb sy");

    // 現在のSCTLR_EL1を読み取り
    let mut sctlr_el1: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);

    // ビット0（M）をクリアしてMMUを無効化
    sctlr_el1 &= !1;

    // SCTLR_EL1に書き戻し
    asm!("msr sctlr_el1, {}", in(reg) sctlr_el1);

    // 変更が適用されることを確認
    asm!("isb");
}

/// 指定された構成でMMUを有効化
pub unsafe fn enable_mmu(config: &MmuConfig) {
    // 変換テーブルを設定
    asm!("msr ttbr0_el1, {}", in(reg) config.ttbr0_el1);
    asm!("msr ttbr1_el1, {}", in(reg) config.ttbr1_el1);

    // 変換制御を構成
    asm!("msr tcr_el1, {}", in(reg) config.tcr_el1);

    // メモリ属性を設定
    asm!("msr mair_el1, {}", in(reg) config.mair_el1);

    // 変更が見えることを確認
    asm!("isb");

    // 現在のSCTLR_EL1を読み取り
    let mut sctlr_el1: u64;
    asm!("mrs {}, sctlr_el1", out(reg) sctlr_el1);

    // ビット0（M）を設定してMMUを有効化
    sctlr_el1 |= 1;

    // SCTLR_EL1に書き戻し
    asm!("msr sctlr_el1, {}", in(reg) sctlr_el1);

    // 変更が適用されることを確認
    asm!("isb");
}

/// メモリ領域のアイデンティティマッピングを作成
pub unsafe fn identity_map_region(
    page_tables: &mut PageTables,
    start_addr: usize,
    size: usize,
    attributes: PageAttributes
) -> Result<(), MapError> {
    let start_page = start_addr / PAGE_SIZE;
    let page_count = (size + PAGE_SIZE - 1) / PAGE_SIZE;

    for i in 0..page_count {
        let virt_addr = (start_page + i) * PAGE_SIZE;
        let phys_addr = virt_addr;  // アイデンティティマッピング

        page_tables.map_page(virt_addr, phys_addr, attributes)?;
    }

    Ok(())
}
```

## 付録B：QEMUデバッグコマンド

デバッグに役立つQEMUモニタコマンドのリファレンスを以下に示します：

```
# 基本的なシステム情報
info registers        # CPUレジスタを表示
info cpus             # CPU状態を表示
info tlb              # TLBコンテンツを表示
info mem              # メモリマッピングを表示

# メモリ検査
x/10i $pc            # プログラムカウンタから10命令を逆アセンブル
x/20wx 0x40010000    # アドレス0x40010000から20ワードを検査
xp/20wx 0x40010000   # 物理アドレス0x40010000から20ワードを検査

# 実行制御
c                     # 実行を継続
s                     # 1命令ステップ実行
p                     # 呼び出し/戻りをステップスルー

# ブレークポイント
b 0x40010120          # アドレスにブレークポイントを設定
watch 0x40020000      # メモリ変更を監視
delete 1              # ブレークポイント#1を削除

# システム制御
system_reset          # システムをリセット
quit                  # QEMUを終了
```

これらのコマンドは、OS開発の旅における低レベルの問題をデバッグする際に非常に価値があります。
