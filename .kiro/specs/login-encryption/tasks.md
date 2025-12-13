# 实现计划

- [x] 1. 添加 Rust 依赖


  - 在 `Cargo.toml` 中添加 `aes-gcm`、`base64`、`rand` 依赖
  - 添加 `proptest` 作为 dev-dependency
  - _需求: 1.1, 2.1_


- [x] 2. 实现加密模块

  - [x] 2.1 创建 `src-tauri/src/crypto.rs` 模块


    - 定义 `CryptoError` 错误枚举
    - 实现 `encrypt` 函数：生成随机 Nonce，AES-GCM 加密，Base64 编码
    - 实现 `decrypt` 函数：Base64 解码，提取 Nonce，AES-GCM 解密
    - _需求: 1.1, 1.2, 1.3, 2.1, 2.2, 2.3, 2.4, 3.1, 3.2, 3.3, 3.4_
  - [ ]* 2.2 编写 Round-trip 属性测试
    - **Property 1: 加密解密往返一致性**
    - **验证: 需求 2.1, 2.2, 2.3, 2.4**
  - [ ]* 2.3 编写输出格式属性测试
    - **Property 2: 加密输出格式正确性**
    - **验证: 需求 1.2, 1.3**
  - [ ]* 2.4 编写 Nonce 随机性属性测试
    - **Property 3: 相同明文产生不同密文**
    - **验证: 需求 1.2**


- [x] 3. 集成加密模块到 Tauri

  - [x] 3.1 在 `lib.rs` 中注册 crypto 模块


    - 添加 `mod crypto;` 声明
    - _需求: 4.1_
  - [x] 3.2 实现 `http_post_encrypted` 命令


    - 在 `utils.rs` 中添加加密 HTTP POST 函数
    - 加密请求体，发送请求，解密响应
    - _需求: 1.4, 4.1, 4.2_

  - [x] 3.3 注册新命令到 Tauri

    - 在 `lib.rs` 的 `invoke_handler` 中添加 `http_post_encrypted`
    - _需求: 4.3_


- [x] 4. 更新前端认证模块

  - [x] 4.1 修改 `auth.ts` 使用加密接口


    - 将 `http_post` 调用改为 `http_post_encrypted`
    - 保持其他逻辑不变
    - _需求: 4.1, 4.2, 4.3_

- [x] 5. Checkpoint - 确保所有测试通过


  - 确保所有测试通过，如有问题请询问用户。


- [x] 6. 最终 Checkpoint - 确保所有测试通过


  - 确保所有测试通过，如有问题请询问用户。
