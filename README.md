# Cantonese SOFA Model Distribution

[English](#cantonese-sofa-model-distribution), [简体中文](#关于本仓库)

## About This Repository

- 1 hour of training data provided by RibosomeK
- 10k training steps
- comprehensive dictionary scheme that can be easily transformed into other schemes
- comes with a python script that transform output files into other schemes, default is `cantonese-two-seg.csv`
- using a customize scheme is possible

---

**Notice that this model only has 1hr of training data from only one provider. Therefor, mis-labeling and label cluster are inevitable. Please lower your expectation and correct those mistakes before use.**

---


## Usages

### SOFA Model

1. download the model with `*.zip` extension from release and extract wherever you like
1. follow the instruction from [SOFA](https://github.com/qiuqiao/SOFA), from `Environment Setup` to `Inference`. Also notice that you should use the dictionary that come with the zip file

## Script

This script is aim to transform label file into other scheme (by default is two-seg) and easy to use.

### Changelogs

2024-12-25

- Fixed missing word in `cantonese-two-seg.csv` (baat)
- Added checking feature, words that already in giving scheme or have mismatch phonemes will be informed or warned. Now you can use the script for the same `TextGrid` file for multiple times

### Usage

```
> python change_scheme.py --help

usage: change_scheme.py [-h] [-s SCHEME] [-o OUT] textgrids

positional arguments:
  textgrids             The directory of textgrid files, non-recursive.

options:
  -h, --help            show this help message and exit
  -s SCHEME, --scheme SCHEME
                        The path of scheme file. default is ./configs/cantonese-two-seg.csv
  -o OUT, --out OUT     output directory. Default is ./out
```

If you're already in a python environment that setup for SOFA, you are ready to go. If not run this to install dependency:

```
pip install -r ./requirements.txt
```

Then you could simply run the following command

```
python change_scheme.py /path/to/your/textgrid/files/ 
```

If you need to change the output directory or scheme, run this:

```
python change_scheme.py /path/to/your/textgrid/files/ --scheme /path/of/your/scheme/config.csv --out /directory/you/wanna/save
```

### Customize Scheme Config

The scheme config is a csv file that indicate how phonemes in a word should be replaced or merged. Each word has twp lines, first line is the original phonemes that should not be changed. the second line is the new phoneme.

Here is an example with only one word (numbers and  bars are demonstration) :

```
               1    2   3    4    5
line 1  bright | b  | r | aa | :i | t 
line 2         | br |   | ay |    | t
```

In this example the word `bright` original has 5 phonemes. New scheme merge `b` and `r` into `br`, `aa` and `:i` into `ay`. In an actual csv file it would be look like this:

```csv
bright,b,r,aa,:i,t
,br,,ay,,t
```

For more reference, you could look into the `./configs/cantonese-two-seg.csv` file.

## Credits

- [SOFA](https://github.com/qiuqiao/SOFA)
- [python textgrid library](https://github.com/kylebgorman/textgrid)


## 关于本仓库

- 由 RibosomeK 提供的 1 小时的训练数据
- 训练步长 10k
- 尽量细分的字典方案，可供转化其他方案
- 自带一个方案转化脚本，默认为二段式 `cantonese-two-seg.csv`
- 可以使用其他方案配置

---

**注意！由于本模型只有一位说话人的一小时数据，标注错误和标记线聚集是不可避免的。请降低推理准确性的预期，并修正后再使用。**

---


## 使用

### SOFA 模型

1. 从 Release 下载文件扩展名为 `*.zip` 的压缩包并解压到你需要的位置
1. 依照 [SOFA](https://github.com/qiuqiao/SOFA) 仓库的 `Environment Setup` 到 `Inference` 的步骤指示执行操作。注意推理时要使用下载压缩包内自带的字典文件

## 脚本

附带的易用脚本用于转化标记文件到不同的方案。

### 更新日志

2024-12-25 

- 补上了 `cantonese-two-seg.csv` 里缺失的字（baat）
- 增加了检查功能。已经被替换的字或错字会被提示或警告，并被跳过。现在可以反复处理同一个 `TextGrid` 文件


### 使用方式

```
> python change_scheme.py --help

usage: change_scheme.py [-h] [-s SCHEME] [-o OUT] textgrids

positional arguments:
  textgrids             The directory of textgrid files, non-recursive.

options:
  -h, --help            show this help message and exit
  -s SCHEME, --scheme SCHEME
                        The path of scheme file. default is ./configs/cantonese-two-seg.csv
  -o OUT, --out OUT     output directory. Default is ./out
```

如果你已经在 SOFA 的 python 环境内，则可直接执行脚本使用，如果不是，请执行以下命令安装依赖：

```
pip install -r ./requirements.txt
```

之后通过以下命令可以运行脚本：

```
python change_scheme.py 存放/textgrid/的/文件夹/
```

如果你需要更改输出目录或者字典方案，请参照以下命令：

```
python change_scheme.py 存放/textgrid/的/文件夹/目录/ --scheme 字典/方案/的/文件/路径 --out 你/需要/的/输出/文件夹
```

### 自定义方案配置

自定义方案配置文件采用 csv 格式。一个配置文件应当包含每一个字里的音素应该怎么被合并或者被替换。每个字有两行，第一行包含了原本的音素方案且不应被更改；第二行则是新的音素方案。

以下举例了仅包含了一个字的范例（数字和竖条仅为示例）：

```
               1    2   3    4    5
行 1  bright | b  | r | aa | :i | t 
行 2         | br |   | ay |    | t
```

举例中 `bright` 原本包含 5 个音素。而在第二行可以看到新的音素方案将 `b` 和 `r` 合并为 `br`，`aa` 和 `:i` 合并为 `ay`。实际在 csv 文件中应为：

```csv
bright,b,r,aa,:i,t
,br,,ay,,t
```

更多例子可以参考 `./configs/cantonese-two-seg.csv`

## 致谢

- [SOFA](https://github.com/qiuqiao/SOFA)
- [python textgrid library](https://github.com/kylebgorman/textgrid)