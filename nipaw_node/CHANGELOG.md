# Changelog

## [1.7.1](https://github.com/puniyu/nipaw/compare/node-v1.7.0...node-v1.7.1) (2025-11-11)


### 🐛 错误修复

* **option:** 将标签和分配用户字段改为可选 ([7ca37b1](https://github.com/puniyu/nipaw/commit/7ca37b1db732707e876d633717602049ac781f3e))


### 🎡 持续集成

* **workflow:** 添加节点标签名输出和上传发布功能 ([d7a27a6](https://github.com/puniyu/nipaw/commit/d7a27a68d8214a518b2183e26d447270229011b1))

## [1.7.0](https://github.com/puniyu/nipaw/compare/node-v1.6.1...node-v1.7.0) (2025-11-11)


### ✨ 新功能

* **core:** 添加更新issue功能并优化依赖管理 ([6c35dca](https://github.com/puniyu/nipaw/commit/6c35dca6dfbe4b324627cf742556fc3c8296f918))
* **core:** 添加获取议题信息功能 ([#47](https://github.com/puniyu/nipaw/issues/47)) ([186d61c](https://github.com/puniyu/nipaw/commit/186d61cc97d0ec4b262c842e1c06ddb2330afa5f))
* **github:** 支持设置 GitHub 反向代理配置 ([da70774](https://github.com/puniyu/nipaw/commit/da7077453d23d1d038f5f5f5e6c644de5fe1be3c))


### 🔧 其他更新

* **deps:** update nipaw packages ([40eba5e](https://github.com/puniyu/nipaw/commit/40eba5e262d02d95747e1397e35303c5cee02d46))
* **deps:** update pnpm to v10.21.0 ([563d181](https://github.com/puniyu/nipaw/commit/563d181e54be65dc5cffd5b232bed6481075b1d7))

## [1.6.1](https://github.com/puniyu/nipaw/compare/node-v1.6.0...node-v1.6.1) (2025-11-07)


### 🐛 错误修复

* **commit:** 添加修改文件数统计字段 ([faf1e3b](https://github.com/puniyu/nipaw/commit/faf1e3b04784f0a6f79d4c4de4129cfac1af162b))


### 🔧 其他更新

* **deps:** update nipaw packages ([7c2f090](https://github.com/puniyu/nipaw/commit/7c2f0907027ed06a0922b6be41b192012d5a3792))

## [1.6.0](https://github.com/puniyu/nipaw/compare/node-v1.5.0...node-v1.6.0) (2025-11-06)


### ✨ 新功能

* **core:** 添加创建议题功能 ([6e270c1](https://github.com/puniyu/nipaw/commit/6e270c1b642d496cae92915506c102f05db4c161))


### 🐛 错误修复

* 修复依赖构建 ([5da7108](https://github.com/puniyu/nipaw/commit/5da71088fb56109d5d3a18d837bd21b5983e896c))


### 🔧 其他更新

* **deps:** pin dependencies ([#33](https://github.com/puniyu/nipaw/issues/33)) ([5fe1af6](https://github.com/puniyu/nipaw/commit/5fe1af6c0704e0cc74dd5a7a1347ae28a20f7316))
* **deps:** update npm packages ([734dcf8](https://github.com/puniyu/nipaw/commit/734dcf8de9bf212890809185ee4208146b7bc4b2))

## [1.5.0](https://github.com/puniyu/nipaw/compare/node-v1.4.1...node-v1.5.0) (2025-11-02)


### ✨ 新功能

* **core:** 引入仓库可见性枚举并优化认证方式 ([040344d](https://github.com/puniyu/nipaw/commit/040344d7d44b5cf1f577735e6c4793274af0295a))


### 🔧 其他更新

* 更新项目元数据和依赖配置 ([7bf12e6](https://github.com/puniyu/nipaw/commit/7bf12e64ae39263769e7237aba6a152b91d1d815))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.8.0 to 0.9.0

## [1.4.1](https://github.com/puniyu-plugins/nipaw/compare/node-v1.4.0...node-v1.4.1) (2025-10-30)


### 🔧 其他更新

* **node:** 更新 nipaw_node 的版本发布配置 ([ffd92c6](https://github.com/puniyu-plugins/nipaw/commit/ffd92c6ffa68c4ae0ddba473388bada1693a921a))
* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu-plugins/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))


### ♻️ 代码重构

* **platform:** 重构平台客户端实现 ([6907f9e](https://github.com/puniyu-plugins/nipaw/commit/6907f9e409da91f2b5b6986bcaa153ba6a72a897))
* **platform:** 重构平台模块代码结构 ([59c2ca6](https://github.com/puniyu-plugins/nipaw/commit/59c2ca6c6795d7ff686c64a8be86a46b1d461c06))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.3.4 to 0.4.0

## [1.4.0](https://github.com/puniyu-plugins/nipaw/compare/node-v1.3.0...node-v1.4.0) (2025-10-08)


### ✨ 新功能

* **core:** 添加仓库协作者功能支持 ([ce87b1c](https://github.com/puniyu-plugins/nipaw/commit/ce87b1cebeb6319096718353082759ca1f0d897b))


### ♻️ 代码重构

* **node:** 简化平台模块中的错误处理类型 ([40a4222](https://github.com/puniyu-plugins/nipaw/commit/40a42225df28f0dac5e4e1d1ef1a31570b193ed0))

## [1.3.0](https://github.com/puniyu-plugins/nipaw/compare/node-v1.2.0...node-v1.3.0) (2025-10-03)


### ✨ 新功能

* **core:** 添加组织信息相关功能支持 ([47730ab](https://github.com/puniyu-plugins/nipaw/commit/47730ab307762f4a63bd3dd6b4007684891df351))
* **repo:** 重构仓库信息结构并优化默认分支获取逻辑 ([2850050](https://github.com/puniyu-plugins/nipaw/commit/28500500c653ec15103b1442270941a59e243af8))


### ♻️ 代码重构

* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu-plugins/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))

## [1.2.0](https://github.com/puniyu-plugins/nipaw/compare/node-v1.1.2...node-v1.2.0) (2025-10-02)


### ✨ 新功能

* **core:** 添加获取仓库提交列表功能 ([0bc8a0a](https://github.com/puniyu-plugins/nipaw/commit/0bc8a0a8ae385cf7d53a2e40c8990f5c89262aac))


### 🐛 错误修复

* **types:** 修复node结构体类型重复 ([c63cc28](https://github.com/puniyu-plugins/nipaw/commit/c63cc28082029d912cd02e36fbf7ad7cea434d76))

## [1.1.2](https://github.com/puniyu-plugins/nipaw/compare/node-v1.1.1...node-v1.1.2) (2025-10-02)


### 🔧 其他更新

* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu-plugins/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))

## [1.1.1](https://github.com/puniyu-plugins/nipaw/compare/node-v1.1.0...node-v1.1.1) (2025-10-02)


### 🐛 错误修复

* 修复导出 ([9e0c3af](https://github.com/puniyu-plugins/nipaw/commit/9e0c3af9d4feec48badf1108a1b405cec9d38e38))

## [1.1.0](https://github.com/puniyu-plugins/nipaw/compare/node-v1.0.2...node-v1.1.0) (2025-10-02)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))
* **platform:** 为多个平台客户端添加代理设置功能 ([ff14fdf](https://github.com/puniyu-plugins/nipaw/commit/ff14fdf6bc78549dcce956e4ee91744dc57a0b0e))


### 🐛 错误修复

* **node:** constructor错误 ([cb996b7](https://github.com/puniyu-plugins/nipaw/commit/cb996b70fb9f57c35e4d886ce39a4632d6def181))


### 🔧 其他更新

* **deps:** 升级 nipaw 系列 crate 至 0.3.0 版本 ([104274b](https://github.com/puniyu-plugins/nipaw/commit/104274b3b62b4e662e1376fcd35b5ec1fcd29e2d))
* **deps:** 更新 nipaw 相关包描述并调整发布流程 ([f064c21](https://github.com/puniyu-plugins/nipaw/commit/f064c211da35fb62f938725406ebe969c320e35b))
* **release:** 移除 nipaw_core 包的发布配置 ([fd0aa9e](https://github.com/puniyu-plugins/nipaw/commit/fd0aa9e595230b9011080736966a3864b53d8419))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **platform:** 统一使用全局tokio运行时 ([0356528](https://github.com/puniyu-plugins/nipaw/commit/03565285d577c6210906691854fdf5be9a8ffd99))
* **platform:** 统一客户端获取方式 ([1926f74](https://github.com/puniyu-plugins/nipaw/commit/1926f747aadedaf960f7750306a736c0c48081df))
