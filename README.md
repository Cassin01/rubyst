# Rubyst
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Ruby Interpreter written in Rust 🦄

# 注意

- 抽象構文木のデータ構造が複雑
- 作者が個人的にRubyの構文よりもPythonの構文の方が好き

の二点の理由からこのリポジトリでの開発は中止しました。
Python構文での再開発を考えています。

## 対応している文法

|型||
| :---:  | :---:|
| bool型 | Bool | 
| int型  | Int  |

|演算子||
| :---:  | :---:|
| 掛け算 | + | 
| 足し算 | - |
| 掛け算 | * |
| 割り算 | / |
| 余り | % |
| 乗算 | ** |

|比較演算子||
| :---:  | :---:|
|イコール|==| 
|大なりイコール|>=|
|小なりイコール|<=|
|大なり|>|
|小なり|<|

|関数||
| :---: | :---:|
| print文 | p() |
| if文 | if ~ end |


## 記述例

```main.eld
(x = 1)
y = x + 1

if y == 2
  p(x + 2)
end

if y < 1
  p(x * 3)
end

p(y % 6)
```

## 開発指針
コードはできるだけ綺麗にするけど開発の速さのためには妥協もする
## このrubystインタプリタで用いられる抽象構文木において

### 望ましいこと
tree.rootがType::Nilであるときtree.leftがOption::Noneであること

&rarr; 実行速度が遅くなるため

### アンチパターン
tree.rootがType::Nilであるときtree.rightがOption::Noneでないこと

&rarr; 複雑度が上がり、開発者が潜在的なバグを予測できなくなるため

## 参考
遠藤侑介 RubyでつくるRuby ラムダノート株式会社
