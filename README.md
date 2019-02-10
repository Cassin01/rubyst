# Rubyst
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Ruby Interpreter written in Rust 🦄
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
## 木構造について
## 望ましいこと
tree.rootがType::Nilであるときtree.leftがOption::Noneであること
## あってはならないこと
tree.rootがType::Nilであるときtree.rightがOption::Some(_)であること

## 参考
遠藤侑介 RubyでつくるRuby ラムダノート株式会社
