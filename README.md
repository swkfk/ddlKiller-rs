# ddlKiller

祝大家远离 DDL 带来的痛苦 QAQ

虽然这个小工具目前尚处于早期开发阶段，并且有一些小 bug，但其命令行界面（`cli_main`）已可以编译使用。

# 代码与编译

该项目将由三个 crate 组成：

- `ddl_core`（`lib`），提供内容管理、文件操作的核心功能，无需单独编译，无法独立使用，为其他两个 crate 提供交互接口
- `cli_main`（`bin`），命令行界面主程序，不计划使用任何 tui 库
- `gui_main`（`bin`），命令行界面主程序，计划使用 `fltk` 作为 gui 库（咕咕咕）

当前版本，仅 `cli_main` 可用，在其目录下，使用

```shell
cargo build --release
```

即可编译，所依赖的所有 crate 会自动下载。

# 名词释义

- `entry` 组织各项 `ddl` 的项目，不同 `entry` 之间的 `ddl` 相互独立
  - `key` 目前版本下，`entry` 的唯一标识符，可以使用中文，但会导致表格输出时发生错位
  - `over` 在代码中被描述为 `enabled`，一个 `entry` 被设置为 `over` 后将不会再被输出，也无法继续添加 `ddl`，相当于索引层面的删除

- `ddl` 直接描述各项事务，在代码中被描述为 `item`
  - `id` 同一个 `entry` 中不同 `ddl` 的唯一标识符，无法手动设置
  - `label` 对该项 `ddl` 的描述，可以使用中文，不会出现输出问题（也许吧）
  - `importance` 分为 5 个等级，数字 0 至 4 为重要性依次增大
  - `over` 表示该项 `ddl` 是否已经完成，会影响输出时文字的颜色

# 命令行界面

## 打印 ddl 信息

命令：`ddlKiller show[/<entry>]`

打印所有或指定 `entry` 中的 `ddl` 信息，按照内置的规则进行排序，以进度条的样式输出。

若不指定 `<entry>`，则分 `entry` 打印全部 `ddl`。

例如：`ddlKiller show` 将打印全部内容；`ddlKiller show/co` 将仅打印 `key` 为 `co` 的 `entry`。

打印时会为每个 `ddl` 打印一个进度条，该进度条表示，从该 `ddl` **被创建** 时刻开始，到现在所经历的时间占全部时间的百分比，当进度在 $80.0\%$ 及以上且该 `ddl` 没有被设置为完成（`over`）时会显示为红色。

## 列举 ddl 信息

命令：`ddlKiller list[/<entry>]`

以更紧凑的方式列举所有或指定 `entry` 中的 `ddl` 信息，会输出详细的 `id`，方便后续使用。

例如：`ddlKiller list` 将列举全部内容；`ddlKiller list/co` 将仅列举 `key` 为 `co` 的 `entry`。

## 新增 entry 或 ddl 信息

命令：`ddlKiller new/[entry|ddl]`

必须指定是新建 `entry` 还是 `ddl`，会进入命令行交互，询问相关信息。

若新建 `entry` 与已有 `entry` 同名，请求将会被拒绝。

## 设置 over

命令：`ddlKiller over[/<entry>[/<ddl-id>]]`

将指定 `entry` 或指定 `entry` 下的指定 `ddl` 设置为 `over`。

将 `entry` 设置为 `over` 会导致其索引被移除，但不会删除本地数据；将 `ddl` 设置为 `over` 仅仅影响打印输出时的排序位与色彩。

例如：`ddlKiller over/test` 会将 `key` 为 `test` 的 `entry` 移除；`ddlKiller over/co/1` 会将 `co` 这个 `entry` 下 `id` 为 `1` 的 `ddl` 设置为已完成的状态。`ddl` 的 `id` 可以通过 `list` 命令查看。

# 本地文件

目前版本仅会在用户目录下建立数据文件，路径为 `${home_dir}/.ddl-killer/`，其路径下，会有若干个 `toml` 文件。

`entry.toml` 记录 `entry` 的信息。

`{%04d}.toml` 记录各个 `entry` 下的 `ddl` 信息。

# 项目综述

项目目的：

- 实用（至少目前已经可以用了，并且效果还可以）
- 作为 rust 学习的阶段练习小项目
- 尝试一些以前没有用过的代码组织形式
- 探索一下 git commit message 的比较合适的写法

同时，当前代码在项目层面上还存在着以下问题：

- `pub` 乱用，严重违背了保护与解耦合的目的
- `ddl_core` 的 `interface` 提供的接口十分混乱，代码量过大
- 自定义错误类型 `DDLError` 被滥用，功能细分不明确
