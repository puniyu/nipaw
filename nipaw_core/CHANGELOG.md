# 变更日志

## [1.9.2](https://github.com/puniyu/nipaw/compare/core-v1.9.1...core-v1.9.2) (2026-01-08)


### 🔧 其他更新

* **deps:** update rust crates ([#88](https://github.com/puniyu/nipaw/issues/88)) ([30aeaf7](https://github.com/puniyu/nipaw/commit/30aeaf7cd5c5bbd16a04c3affb3fcec227d1288d))

## [1.9.1](https://github.com/puniyu/nipaw/compare/core-v1.9.0...core-v1.9.1) (2025-12-27)


### 🐛 错误修复

* **commit:** 添加 CommitListInfo 类型以支持提交列表功能 ([037d40b](https://github.com/puniyu/nipaw/commit/037d40bba1a5b591438ff404f18d6145930f0a03))

## [1.9.0](https://github.com/puniyu/nipaw/compare/core-v0.11.1...core-v1.9.0) (2025-12-13)


### ✨ 新功能

* **core:** 引入 Release 模块并重构 Commit 相关类型 ([6598fc4](https://github.com/puniyu/nipaw/commit/6598fc42493098978046d0c7b784e8bb1529355d))


### ♻️ 代码重构

* **core:** 重构选项模块结构 ([17fb9b1](https://github.com/puniyu/nipaw/commit/17fb9b1cfed7420ff8c9cf285373999f2a678cc4))

## [0.11.1](https://github.com/puniyu/nipaw/compare/core-v0.11.0...core-v0.11.1) (2025-11-11)


### 🐛 错误修复

* **option:** 将标签和分配用户字段改为可选 ([7ca37b1](https://github.com/puniyu/nipaw/commit/7ca37b1db732707e876d633717602049ac781f3e))

## [0.11.0](https://github.com/puniyu/nipaw/compare/core-v0.10.1...core-v0.11.0) (2025-11-11)


### ✨ 新功能

* **core:** 添加更新issue功能并优化依赖管理 ([6c35dca](https://github.com/puniyu/nipaw/commit/6c35dca6dfbe4b324627cf742556fc3c8296f918))
* **core:** 添加获取议题信息功能 ([#47](https://github.com/puniyu/nipaw/issues/47)) ([186d61c](https://github.com/puniyu/nipaw/commit/186d61cc97d0ec4b262c842e1c06ddb2330afa5f))

## [0.10.1](https://github.com/puniyu/nipaw/compare/core-v0.10.0...core-v0.10.1) (2025-11-07)


### 🐛 错误修复

* **commit:** 添加修改文件数统计字段 ([faf1e3b](https://github.com/puniyu/nipaw/commit/faf1e3b04784f0a6f79d4c4de4129cfac1af162b))

## [0.10.0](https://github.com/puniyu/nipaw/compare/core-v0.9.0...core-v0.10.0) (2025-11-06)


### ✨ 新功能

* **core:** 添加创建议题功能 ([6e270c1](https://github.com/puniyu/nipaw/commit/6e270c1b642d496cae92915506c102f05db4c161))


### 🐛 错误修复

* **auth:** 细化 Forbidden 错误信息处理 ([c611a41](https://github.com/puniyu/nipaw/commit/c611a418d822965c365811fe0a328979547b2db8))


### 🔧 其他更新

* **deps:** pin dependencies ([#33](https://github.com/puniyu/nipaw/issues/33)) ([5fe1af6](https://github.com/puniyu/nipaw/commit/5fe1af6c0704e0cc74dd5a7a1347ae28a20f7316))

## [0.9.0](https://github.com/puniyu/nipaw/compare/core-v0.8.0...core-v0.9.0) (2025-11-02)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 引入仓库可见性枚举并优化认证方式 ([040344d](https://github.com/puniyu/nipaw/commit/040344d7d44b5cf1f577735e6c4793274af0295a))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库协作者功能支持 ([ce87b1c](https://github.com/puniyu/nipaw/commit/ce87b1cebeb6319096718353082759ca1f0d897b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加组织信息相关功能支持 ([47730ab](https://github.com/puniyu/nipaw/commit/47730ab307762f4a63bd3dd6b4007684891df351))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库提交列表功能 ([0bc8a0a](https://github.com/puniyu/nipaw/commit/0bc8a0a8ae385cf7d53a2e40c8990f5c89262aac))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **repo:** 重构仓库信息结构并优化默认分支获取逻辑 ([2850050](https://github.com/puniyu/nipaw/commit/28500500c653ec15103b1442270941a59e243af8))


### 🔧 其他更新

* release main ([c3174d2](https://github.com/puniyu/nipaw/commit/c3174d2a313f0620d80f377d6000cc6c2baf3a4a))
* release main ([01f9d1d](https://github.com/puniyu/nipaw/commit/01f9d1dc7cc91edd7eec22d4989dcb2d84bcebf2))
* release main ([889df11](https://github.com/puniyu/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([cf4843c](https://github.com/puniyu/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))
* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* 更新项目元数据和依赖配置 ([7bf12e6](https://github.com/puniyu/nipaw/commit/7bf12e64ae39263769e7237aba6a152b91d1d815))


### ♻️ 代码重构

* **core:** 简化Result类型使用并优化模块导出 ([aad14e1](https://github.com/puniyu/nipaw/commit/aad14e1f9a0c21e413bc2d457f4c55f507ec1b68))
* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))

## [0.4.0](https://github.com/puniyu-plugins/nipaw/compare/core-v0.3.4...core-v0.4.0) (2025-10-30)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加仓库协作者功能支持 ([ce87b1c](https://github.com/puniyu-plugins/nipaw/commit/ce87b1cebeb6319096718353082759ca1f0d897b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加组织信息相关功能支持 ([47730ab](https://github.com/puniyu-plugins/nipaw/commit/47730ab307762f4a63bd3dd6b4007684891df351))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库提交列表功能 ([0bc8a0a](https://github.com/puniyu-plugins/nipaw/commit/0bc8a0a8ae385cf7d53a2e40c8990f5c89262aac))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))
* **repo:** 重构仓库信息结构并优化默认分支获取逻辑 ([2850050](https://github.com/puniyu-plugins/nipaw/commit/28500500c653ec15103b1442270941a59e243af8))


### 🔧 其他更新

* release main ([01f9d1d](https://github.com/puniyu-plugins/nipaw/commit/01f9d1dc7cc91edd7eec22d4989dcb2d84bcebf2))
* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu-plugins/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))
* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu-plugins/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))


### ♻️ 代码重构

* **core:** 简化Result类型使用并优化模块导出 ([aad14e1](https://github.com/puniyu-plugins/nipaw/commit/aad14e1f9a0c21e413bc2d457f4c55f507ec1b68))
* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu-plugins/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))

## [0.3.0](https://github.com/puniyu-plugins/nipaw/compare/v0.2.1...v0.3.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))


### 🔧 其他更新

* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))

## [0.2.1](https://github.com/puniyu-plugins/nipaw/compare/v0.2.0...v0.2.1) (2025-10-01)


### 🔧 其他更新

* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))

## [0.2.0](https://github.com/puniyu-plugins/nipaw/compare/v0.1.0...v0.2.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))
* **nipaw_node:** 初始化 Node.js 绑定模块 ([0917a1d](https://github.com/puniyu-plugins/nipaw/commit/0917a1d1623e6bca98f78da00546806f21a9d113))


### 🔧 其他更新

* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
