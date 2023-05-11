# はじめに必要だったこと
```
ERROR: the sysroot can't be built for the Stable channel. Switch to nightly.
```
というエラーが出た
<br>
```
rustup toolchain install nightly
rustup override set nightly
```
上記を実行したら解決した

# xargo
xargoからcargoに変えようとしたら、i386-unknown-noneのcoreが見つからない的なエラーが出た。
-Zbuild-stdをつけてコンパイルしようとしたら、そのライブラリが見つからないというようなエラーが出た
