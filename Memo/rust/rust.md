# mod
自分と同じディレクトリ以外のファイルを読み込むときはmodを使うっぽい

# static変数
プログラム全体のlifetimeの間、値を保持する

# ビルドオプション
```  cargo build -Z build-std=core,compiler_builtins -Z build-std-features=compiler-builtins-mem --target $(CUSTOM_TARGET).json```

.cargo/config.tomlに以下を追加すると、コンパイル時に上記のオプションが付与される
```
[build]
target = "./kernel/i386-yamaos.json"

[unstable]
build-std = ["core", "compiler_builtins"]
build-std-features = ["compiler-builtins-mem"]
```