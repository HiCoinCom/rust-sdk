# ChainUp Custody Rust SDK

[![Crates.io](https://img.shields.io/crates/v/chainup-custody-sdk.svg)](https://crates.io/crates/chainup-custody-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[English](#english) | [中文](#中文)

---

## English

### Overview

ChainUp Custody Rust SDK provides a simple and secure way to integrate with ChainUp Custody's WaaS (Wallet as a Service) and MPC (Multi-Party Computation) APIs.

### Features

- 🔐 **RSA Encryption** - Secure communication with RSA encryption/decryption
- 🔑 **Digital Signature** - MD5 + SHA256 + RSA signature for request authentication
- 💼 **WaaS API** - Full support for Wallet as a Service operations
- 🔒 **MPC API** - Full support for MPC wallet operations
- 📦 **Easy Integration** - Simple builder pattern for client initialization

### Requirements

- Cargo 1.83+

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
chainup-custody-sdk = { git = "https://github.com/HiCoinCom/rust-sdk.git", branch = "main" }

```

### Quick Start

#### MPC Client

```rust
use chainup_custody_sdk::{error::ChainUpError, mpc::MpcClient};

fn main() -> Result<(), ChainUpError> {
    // Create MPC client
    let client = MpcClient::builder()
        .set_app_id("your_app_id")
        .set_rsa_private_key("your_rsa_private_key")
        .set_waas_public_key("waas_public_key")
        .set_sign_private_key("your_sign_private_key")
        .set_debug(true)
        .build()?;

    // Get wallet API
    let wallet_api = client.get_wallet_api();

    // Query wallet assets
    use chainup_custody_sdk::mpc_types::GetWalletAssetsParams;
    let params = GetWalletAssetsParams::new(1000537, "ETH");
    let assets = wallet_api.get_wallet_assets(params)?;
    println!("Assets: {:?}", assets);

    Ok(())
}
```

#### WaaS Client

```rust
use chainup_custody_sdk::{error::ChainUpError, waas::WaasClient};

fn main() -> Result<(), ChainUpError> {
    // Create WaaS client
    let client = WaasClient::builder()
        .set_app_id("your_app_id")
        .set_private_key("your_private_key")
        .set_public_key("waas_public_key")
        .set_debug(true)
        .build()?;

    // Get user API
    let user_api = client.get_user_api();

    // Register email user
    use chainup_custody_sdk::waas_types::RegisterEmailUserParams;
    let params = RegisterEmailUserParams::new("user@example.com");
    let user = user_api.register_email_user(params)?;
    println!("User ID: {}", user.uid);

    Ok(())
}
```

### API Reference

#### MPC APIs

| API Category          | Methods                                                                                                            |
| --------------------- | ------------------------------------------------------------------------------------------------------------------ |
| **Wallet API**        | `create_wallet`, `create_wallet_address`, `query_wallet_address`, `get_wallet_assets`, `change_wallet_show_status` |
| **Deposit API**       | `get_deposit_records`, `get_last_block_height`, `sync_deposit_records`                                             |
| **Withdraw API**      | `withdraw`, `sync_withdraw_records`                                                                                |
| **Web3 API**          | `create_web3_trans`, `accelerate_web3_trans`, `sync_web3_records`                                                  |
| **Workspace API**     | `get_coin_details`, `get_all_main_symbols`, `get_sub_wallets`                                                      |
| **Auto Sweep API**    | `auto_collect_sub_wallets`, `set_auto_collect_symbol`, `get_auto_collect_symbols`                                  |
| **Tron Resource API** | `create_tron_delegate`, `get_tron_account_resource`                                                                |
| **Notify API**        | `wallet_address_info`                                                                                              |

#### WaaS APIs

| API Category     | Methods                                                                                                     |
| ---------------- | ----------------------------------------------------------------------------------------------------------- |
| **User API**     | `register_mobile_user`, `register_email_user`, `get_mobile_user`, `get_email_user`, `sync_user_list`        |
| **Account API**  | `get_user_account`, `get_user_address`, `get_user_address_info`, `get_company_account`, `sync_address_list` |
| **Transfer API** | `account_transfer`, `get_account_transfer_list`                                                             |
| **Billing API**  | `withdraw`, `get_withdraw_list`, `sync_withdraw_list`, `sync_deposit_list`                                  |

### Examples

Run the example:

```bash
# MPC example
cargo run --example mpc_example

# WaaS example
cargo run --example waas_example
```

### License

MIT License

---

## 中文

### 概述

ChainUp Custody Rust SDK 提供了一种简单安全的方式来集成 ChainUp Custody 的 WaaS（钱包即服务）和 MPC（多方计算）API。

### 特性

- 🔐 **RSA 加密** - 使用 RSA 加密/解密进行安全通信
- 🔑 **数字签名** - 使用 MD5 + SHA256 + RSA 签名进行请求认证
- 💼 **WaaS API** - 完整支持钱包即服务操作
- 🔒 **MPC API** - 完整支持 MPC 钱包操作
- 📦 **易于集成** - 使用 Builder 模式简化客户端初始化

### 环境要求

- Cargo 1.83+

### 安装

在 `Cargo.toml` 中添加：

```toml
[dependencies]
chainup-custody-sdk = { git = "https://github.com/HiCoinCom/rust-sdk.git", branch = "main" }
```

### 快速开始

#### MPC 客户端

```rust
use chainup_custody_sdk::{error::ChainUpError, mpc::MpcClient};

fn main() -> Result<(), ChainUpError> {
    // 创建 MPC 客户端
    let client = MpcClient::builder()
        .set_app_id("your_app_id")
        .set_rsa_private_key("your_rsa_private_key")
        .set_waas_public_key("waas_public_key")
        .set_sign_private_key("your_sign_private_key")
        .set_debug(true)
        .build()?;

    // 获取钱包 API
    let wallet_api = client.get_wallet_api();

    // 查询钱包资产
    use chainup_custody_sdk::mpc_types::GetWalletAssetsParams;
    let params = GetWalletAssetsParams::new(1000537, "ETH");
    let assets = wallet_api.get_wallet_assets(params)?;
    println!("资产: {:?}", assets);

    Ok(())
}
```

#### WaaS 客户端

```rust
use chainup_custody_sdk::{error::ChainUpError, waas::WaasClient};

fn main() -> Result<(), ChainUpError> {
    // 创建 WaaS 客户端
    let client = WaasClient::builder()
        .set_app_id("your_app_id")
        .set_private_key("your_private_key")
        .set_public_key("waas_public_key")
        .set_debug(true)
        .build()?;

    // 获取用户 API
    let user_api = client.get_user_api();

    // 注册邮箱用户
    use chainup_custody_sdk::waas_types::RegisterEmailUserParams;
    let params = RegisterEmailUserParams::new("user@example.com");
    let user = user_api.register_email_user(params)?;
    println!("用户 ID: {}", user.uid);

    Ok(())
}
```

### API 参考

#### MPC API

| API 分类          | 方法                                                                                                               |
| ----------------- | ------------------------------------------------------------------------------------------------------------------ |
| **钱包 API**      | `create_wallet`, `create_wallet_address`, `query_wallet_address`, `get_wallet_assets`, `change_wallet_show_status` |
| **充值 API**      | `get_deposit_records`, `get_last_block_height`, `sync_deposit_records`                                             |
| **提现 API**      | `withdraw`, `sync_withdraw_records`                                                                                |
| **Web3 API**      | `create_web3_trans`, `accelerate_web3_trans`, `sync_web3_records`                                                  |
| **工作空间 API**  | `get_coin_details`, `get_all_main_symbols`, `get_sub_wallets`                                                      |
| **自动归集 API**  | `auto_collect_sub_wallets`, `set_auto_collect_symbol`, `get_auto_collect_symbols`                                  |
| **Tron 资源 API** | `create_tron_delegate`, `get_tron_account_resource`                                                                |
| **通知 API**      | `wallet_address_info`                                                                                              |

#### WaaS API

| API 分类     | 方法                                                                                                        |
| ------------ | ----------------------------------------------------------------------------------------------------------- |
| **用户 API** | `register_mobile_user`, `register_email_user`, `get_mobile_user`, `get_email_user`, `sync_user_list`        |
| **账户 API** | `get_user_account`, `get_user_address`, `get_user_address_info`, `get_company_account`, `sync_address_list` |
| **转账 API** | `account_transfer`, `get_account_transfer_list`                                                             |
| **账单 API** | `withdraw`, `get_withdraw_list`, `sync_withdraw_list`, `sync_deposit_list`                                  |

### 示例

运行示例：

```bash
# MPC 示例
cargo run --example mpc_example

# WaaS 示例
cargo run --example waas_example
```

### 签名算法说明

SDK 使用以下签名流程确保请求安全：

1. **参数排序** - 按 ASCII 升序对参数键进行排序
2. **拼接字符串** - 将键值对用 `&` 连接，转换为小写
3. **MD5 哈希** - 对拼接字符串进行 MD5 哈希，得到 32 位十六进制字符串
4. **SHA256 哈希** - 对 MD5 结果进行 SHA256 哈希
5. **RSA 签名** - 使用 PKCS1v15 对 SHA256 结果进行 RSA 签名
6. **Base64 编码** - 将签名结果进行 Base64 编码

### 许可证

MIT License
