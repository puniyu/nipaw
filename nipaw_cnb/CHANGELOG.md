# 变更日志

## [1.9.1](https://github.com/puniyu/nipaw/compare/cnb-v1.9.0...cnb-v1.9.1) (2025-12-27)


### 🐛 错误修复

* **commit:** 添加 CommitListInfo 类型以支持提交列表功能 ([037d40b](https://github.com/puniyu/nipaw/commit/037d40bba1a5b591438ff404f18d6145930f0a03))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 1.9.0 to 1.9.1

## [1.9.0](https://github.com/puniyu/nipaw/compare/cnb-v0.12.1...cnb-v1.9.0) (2025-12-13)


### ✨ 新功能

* **core:** 引入 Release 模块并重构 Commit 相关类型 ([6598fc4](https://github.com/puniyu/nipaw/commit/6598fc42493098978046d0c7b784e8bb1529355d))


### ♻️ 代码重构

* **core:** 重构选项模块结构 ([17fb9b1](https://github.com/puniyu/nipaw/commit/17fb9b1cfed7420ff8c9cf285373999f2a678cc4))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.12.2 to 1.9.0

## [0.12.1](https://github.com/puniyu/nipaw/compare/cnb-v0.12.0...cnb-v0.12.1) (2025-12-11)


### 🐛 错误修复

* **api:** 调整默认分页参数以提高一致性 ([ef96dbe](https://github.com/puniyu/nipaw/commit/ef96dbeecefee60d053775af5736fefb9fc5cc6b))
* **common:** 过滤空字符串字段值 ([49d52d7](https://github.com/puniyu/nipaw/commit/49d52d7469cf38a0439bf5d76929313c8def7752))

## [0.12.0](https://github.com/puniyu/nipaw/compare/cnb-v0.11.1...cnb-v0.12.0) (2025-11-11)


### ✨ 新功能

* **http:** 引入 tokio 并重构 HTTP 客户端实现 ([95d27a3](https://github.com/puniyu/nipaw/commit/95d27a314eaf7a1f6bff80e388717fd61368aa93))

## [0.11.1](https://github.com/puniyu/nipaw/compare/cnb-v0.11.0...cnb-v0.11.1) (2025-11-11)


### 🐛 错误修复

* **option:** 将标签和分配用户字段改为可选 ([7ca37b1](https://github.com/puniyu/nipaw/commit/7ca37b1db732707e876d633717602049ac781f3e))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.11.0 to 0.11.1

## [0.11.0](https://github.com/puniyu/nipaw/compare/cnb-v0.10.1...cnb-v0.11.0) (2025-11-11)


### ✨ 新功能

* **core:** 添加更新issue功能并优化依赖管理 ([6c35dca](https://github.com/puniyu/nipaw/commit/6c35dca6dfbe4b324627cf742556fc3c8296f918))
* **core:** 添加获取议题信息功能 ([#47](https://github.com/puniyu/nipaw/issues/47)) ([186d61c](https://github.com/puniyu/nipaw/commit/186d61cc97d0ec4b262c842e1c06ddb2330afa5f))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.10.1 to 0.11.0

## [0.10.1](https://github.com/puniyu/nipaw/compare/cnb-v0.10.0...cnb-v0.10.1) (2025-11-07)


### 🐛 错误修复

* **commit:** 添加修改文件数统计字段 ([faf1e3b](https://github.com/puniyu/nipaw/commit/faf1e3b04784f0a6f79d4c4de4129cfac1af162b))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.10.0 to 0.10.1

## [0.10.0](https://github.com/puniyu/nipaw/compare/cnb-v0.9.0...cnb-v0.10.0) (2025-11-06)


### ✨ 新功能

* **core:** 添加创建议题功能 ([6e270c1](https://github.com/puniyu/nipaw/commit/6e270c1b642d496cae92915506c102f05db4c161))


### 🐛 错误修复

* **auth:** 细化 Forbidden 错误信息处理 ([c611a41](https://github.com/puniyu/nipaw/commit/c611a418d822965c365811fe0a328979547b2db8))
* **nipaw_cnb:** 更新cnb.coll默认分支获取逻辑以支持新API格式 ([064e0df](https://github.com/puniyu/nipaw/commit/064e0df39db89ae19289528c629ece957c967dfd))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.9.0 to 0.10.0

## [0.9.0](https://github.com/puniyu/nipaw/compare/cnb-v0.8.0...cnb-v0.9.0) (2025-11-02)


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

* **deps:** 升级 nipaw 系列 crate 至 0.3.0 版本 ([104274b](https://github.com/puniyu/nipaw/commit/104274b3b62b4e662e1376fcd35b5ec1fcd29e2d))
* **deps:** 更新 nipaw 相关包描述并调整发布流程 ([f064c21](https://github.com/puniyu/nipaw/commit/f064c211da35fb62f938725406ebe969c320e35b))
* release main ([c3174d2](https://github.com/puniyu/nipaw/commit/c3174d2a313f0620d80f377d6000cc6c2baf3a4a))
* release main ([be2d330](https://github.com/puniyu/nipaw/commit/be2d33040bf732f12996a4d08953231456826c20))
* release main ([93c8c20](https://github.com/puniyu/nipaw/commit/93c8c201910801a6168efdf4437fcd0bd2bd3f6f))
* release main ([9b70f03](https://github.com/puniyu/nipaw/commit/9b70f03ff59b69d2abb9316c69510d8b1ff4a26b))
* release main ([2c871f8](https://github.com/puniyu/nipaw/commit/2c871f89f528598061f55c0f2fd2b10ee987bad7))
* release main ([135147c](https://github.com/puniyu/nipaw/commit/135147c76f9e0777dfb8f5f43291f55a4d6486c4))
* release main ([16561a2](https://github.com/puniyu/nipaw/commit/16561a2c92ea69ac83d910cbbfbe1ccbb50b02c0))
* release main ([8d71624](https://github.com/puniyu/nipaw/commit/8d716248b58a97ecfe3655e18afeff107b32cd2a))
* release main ([ed9335c](https://github.com/puniyu/nipaw/commit/ed9335cd73868144b16f9cc37c1fb5bb415d19d1))
* release main ([01f9d1d](https://github.com/puniyu/nipaw/commit/01f9d1dc7cc91edd7eec22d4989dcb2d84bcebf2))
* release main ([889df11](https://github.com/puniyu/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([0611059](https://github.com/puniyu/nipaw/commit/0611059583fef44ba9bb2678477e7b63dad89a32))
* release main ([cf4843c](https://github.com/puniyu/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 移除 nipaw_core 包的发布配置 ([fd0aa9e](https://github.com/puniyu/nipaw/commit/fd0aa9e595230b9011080736966a3864b53d8419))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))
* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* **workflow:** 更新下载构建产物的模式匹配规则 ([13e8450](https://github.com/puniyu/nipaw/commit/13e845074d762b555499b122c86d7968ca83df30))
* 更新项目元数据和依赖配置 ([7bf12e6](https://github.com/puniyu/nipaw/commit/7bf12e64ae39263769e7237aba6a152b91d1d815))


### ♻️ 代码重构

* **core:** 简化Result类型使用并优化模块导出 ([aad14e1](https://github.com/puniyu/nipaw/commit/aad14e1f9a0c21e413bc2d457f4c55f507ec1b68))
* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.8.0 to 0.9.0

## [0.7.1](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.7.0...cnb-v0.7.1) (2025-10-30)


### 🔧 其他更新

* **release:** 配置 release-please 支持 Rust 项目 ([69873f0](https://github.com/puniyu-plugins/nipaw/commit/69873f0ddc696958d6b4905611fcf155c0feeea8))


### Dependencies

* The following workspace dependencies were updated
  * dependencies
    * nipaw_core bumped from 0.3.4 to 0.4.0

## [0.7.0](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.6.0...cnb-v0.7.0) (2025-10-08)


### ✨ 新功能

* **core:** 添加仓库协作者功能支持 ([ce87b1c](https://github.com/puniyu-plugins/nipaw/commit/ce87b1cebeb6319096718353082759ca1f0d897b))

## [0.6.0](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.5.0...cnb-v0.6.0) (2025-10-03)


### ✨ 新功能

* **core:** 添加组织信息相关功能支持 ([47730ab](https://github.com/puniyu-plugins/nipaw/commit/47730ab307762f4a63bd3dd6b4007684891df351))
* **repo:** 重构仓库信息结构并优化默认分支获取逻辑 ([2850050](https://github.com/puniyu-plugins/nipaw/commit/28500500c653ec15103b1442270941a59e243af8))


### ♻️ 代码重构

* **core:** 简化Result类型使用并优化模块导出 ([aad14e1](https://github.com/puniyu-plugins/nipaw/commit/aad14e1f9a0c21e413bc2d457f4c55f507ec1b68))
* **core:** 重命名 CoreError为 Error 并更新相关引用 ([0d2f8c4](https://github.com/puniyu-plugins/nipaw/commit/0d2f8c44e654f0f2640929d20b98dbb85c8b7b60))

## [0.5.0](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.4.1...cnb-v0.5.0) (2025-10-02)


### ✨ 新功能

* **core:** 添加获取仓库提交列表功能 ([0bc8a0a](https://github.com/puniyu-plugins/nipaw/commit/0bc8a0a8ae385cf7d53a2e40c8990f5c89262aac))

## [0.4.1](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.4.0...cnb-v0.4.1) (2025-10-02)


### 🔧 其他更新

* **user:** 将用户昵称字段改为可选 ([8c9fac1](https://github.com/puniyu-plugins/nipaw/commit/8c9fac1aa0f47e825b8665ed4f0bb69c84a2b201))

## [0.4.0](https://github.com/puniyu-plugins/nipaw/compare/cnb-v0.3.3...cnb-v0.4.0) (2025-10-02)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))


### 🔧 其他更新

* **deps:** 升级 nipaw 系列 crate 至 0.3.0 版本 ([104274b](https://github.com/puniyu-plugins/nipaw/commit/104274b3b62b4e662e1376fcd35b5ec1fcd29e2d))
* **deps:** 更新 nipaw 相关包描述并调整发布流程 ([f064c21](https://github.com/puniyu-plugins/nipaw/commit/f064c211da35fb62f938725406ebe969c320e35b))
* release main ([16561a2](https://github.com/puniyu-plugins/nipaw/commit/16561a2c92ea69ac83d910cbbfbe1ccbb50b02c0))
* release main ([8d71624](https://github.com/puniyu-plugins/nipaw/commit/8d716248b58a97ecfe3655e18afeff107b32cd2a))
* release main ([ed9335c](https://github.com/puniyu-plugins/nipaw/commit/ed9335cd73868144b16f9cc37c1fb5bb415d19d1))
* release main ([01f9d1d](https://github.com/puniyu-plugins/nipaw/commit/01f9d1dc7cc91edd7eec22d4989dcb2d84bcebf2))
* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([0611059](https://github.com/puniyu-plugins/nipaw/commit/0611059583fef44ba9bb2678477e7b63dad89a32))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 移除 nipaw_core 包的发布配置 ([fd0aa9e](https://github.com/puniyu-plugins/nipaw/commit/fd0aa9e595230b9011080736966a3864b53d8419))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* **workflow:** 更新下载构建产物的模式匹配规则 ([13e8450](https://github.com/puniyu-plugins/nipaw/commit/13e845074d762b555499b122c86d7968ca83df30))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))

## [0.3.3](https://github.com/puniyu-plugins/nipaw/compare/v0.3.2...v0.3.3) (2025-10-02)


### 🔧 其他更新

* **deps:** 更新 nipaw 相关包描述并调整发布流程 ([f064c21](https://github.com/puniyu-plugins/nipaw/commit/f064c211da35fb62f938725406ebe969c320e35b))

## [0.3.2](https://github.com/puniyu-plugins/nipaw/compare/v0.3.1...v0.3.2) (2025-10-01)


### 🔧 其他更新

* **release:** 移除 nipaw_core 包的发布配置 ([fd0aa9e](https://github.com/puniyu-plugins/nipaw/commit/fd0aa9e595230b9011080736966a3864b53d8419))

## [0.3.1](https://github.com/puniyu-plugins/nipaw/compare/v0.3.0...v0.3.1) (2025-10-01)


### 🔧 其他更新

* **deps:** 升级 nipaw 系列 crate 至 0.3.0 版本 ([104274b](https://github.com/puniyu-plugins/nipaw/commit/104274b3b62b4e662e1376fcd35b5ec1fcd29e2d))

## [0.3.0](https://github.com/puniyu-plugins/nipaw/compare/v0.2.2...v0.3.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))


### 🔧 其他更新

* release main ([889df11](https://github.com/puniyu-plugins/nipaw/commit/889df11713dce70094ab9715f772866516ed1277))
* release main ([0611059](https://github.com/puniyu-plugins/nipaw/commit/0611059583fef44ba9bb2678477e7b63dad89a32))
* release main ([cf4843c](https://github.com/puniyu-plugins/nipaw/commit/cf4843cef525bd11cbf52967b0a15741d8a9e726))
* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))
* **workflow:** 更新下载构建产物的模式匹配规则 ([13e8450](https://github.com/puniyu-plugins/nipaw/commit/13e845074d762b555499b122c86d7968ca83df30))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))

## [0.2.2](https://github.com/puniyu-plugins/nipaw/compare/v0.2.1...v0.2.2) (2025-10-01)


### 🔧 其他更新

* **workflows:** 调整构建和发布工作流配置 ([8d6b6d7](https://github.com/puniyu-plugins/nipaw/commit/8d6b6d7fc9994bbd832afd9ee010b88513c1e5e8))

## [0.2.1](https://github.com/puniyu-plugins/nipaw/compare/v0.2.0...v0.2.1) (2025-10-01)


### 🔧 其他更新

* **workflow:** 更新下载构建产物的模式匹配规则 ([13e8450](https://github.com/puniyu-plugins/nipaw/commit/13e845074d762b555499b122c86d7968ca83df30))

## [0.2.0](https://github.com/puniyu-plugins/nipaw/compare/v0.1.0...v0.2.0) (2025-10-01)


### ✨ 新功能

* **client:** 添加获取用户头像URL功能 ([e02321d](https://github.com/puniyu-plugins/nipaw/commit/e02321d7eee5e225fb4e235148643031496f1b11))
* **core:** 支持通过token控制获取仓库默认分支的方式 ([6c729de](https://github.com/puniyu-plugins/nipaw/commit/6c729dec53f0d6e29263e22344c67c88721b517d))
* **core:** 添加仓库信息获取功能 ([c0f1114](https://github.com/puniyu-plugins/nipaw/commit/c0f1114af7764e6a7e1362edbafe04721119639b))
* **core:** 添加用户贡献数据和仓库列表功能 ([ebc8947](https://github.com/puniyu-plugins/nipaw/commit/ebc894715d67d6a14c3385ccbe6c786f48c080bd))
* **core:** 添加获取仓库提交信息功能 ([e54aca3](https://github.com/puniyu-plugins/nipaw/commit/e54aca38e6f5b68a34f0729e4f1052cc31d50f6e))
* **core:** 添加获取仓库默认分支功能 ([8dead32](https://github.com/puniyu-plugins/nipaw/commit/8dead321fe0aae917d08ea61fa64a3d64c2c56e3))
* **gitcode:** 添加获取用户头像URL功能 ([cae522d](https://github.com/puniyu-plugins/nipaw/commit/cae522d36232bd45f9bcd22a3a774c4383e760d5))


### 🔧 其他更新

* **release:** 配置 release-please 和发布工作流 ([5b2700c](https://github.com/puniyu-plugins/nipaw/commit/5b2700c2155645a6fd5625c9514e3bb89b484307))
* 初始化仓库 ([ba51774](https://github.com/puniyu-plugins/nipaw/commit/ba517747af1ca817786475db2bf15ad753d91000))


### ♻️ 代码重构

* **core:** 统一数据转换逻辑并优化依赖管理 ([8765bf8](https://github.com/puniyu-plugins/nipaw/commit/8765bf8e6b483ee10ab723efb01e7476cccc1ff4))
