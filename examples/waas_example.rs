//! WaaS API 使用示例
//!
//! 运行命令: cargo run --example waas_example

use chainup_custody_sdk::{
    enums::QueryIdType,
    error::ChainUpError,
    waas::WaasClient,
    waas_types::{
        // Transfer API
        AccountTransferParams,
        GetAccountTransferListParams,
        GetCompanyAccountParams,
        GetEmailUserParams,
        GetMobileUserParams,
        // Account API
        GetUserAccountParams,
        GetUserAddressInfoParams,
        GetUserAddressParams,
        RegisterEmailUserParams,
        // User API
        RegisterMobileUserParams,
        // Billing API
        WithdrawParams,
    },
};

fn main() -> Result<(), ChainUpError> {
    println!("{}", "=".repeat(60));
    println!("ChainUp Custody Rust SDK - WaaS 示例");
    println!("{}", "=".repeat(60));

    // ============== 配置客户端 ==============
    // 请替换为你的实际配置
    let app_id = "";
    let private_key = r#""#;
    let public_key = r#""#;

    // 使用 Builder 模式创建客户端
    let client = WaasClient::builder()
        .set_app_id(app_id)
        .set_private_key(private_key)
        .set_public_key(public_key)
        .set_debug(false)
        .build()?;

    println!("\n✓ WaaS 客户端创建成功");

    let uid = 15036904_i64; // 替换为实际用户ID

    // ============== 用户管理 (User API) ==============
    println!("\n{}", "=".repeat(40));
    println!("用户管理 (User API)");
    println!("{}", "=".repeat(40));

    let user_api = client.get_user_api();

    // 1. 注册手机用户
    let mobile_params = RegisterMobileUserParams::new("86", "13880000000");
    match user_api.register_mobile_user(mobile_params) {
        Ok(user) => println!("✓ 注册手机用户: uid={}", user.uid),
        Err(e) => println!("✗ 注册手机用户失败: {}", e),
    }

    // 2. 注册邮箱用户
    let email_params = RegisterEmailUserParams::new("user98@example.com");
    match user_api.register_email_user(email_params) {
        Ok(user) => println!("✓ 注册邮箱用户: uid={}", user.uid),
        Err(e) => println!("✗ 注册邮箱用户失败: {}", e),
    }

    // 3. 获取手机用户信息
    let get_mobile_params = GetMobileUserParams::new("86", "13800000000");
    match user_api.get_mobile_user(get_mobile_params) {
        Ok(user) => println!("✓ 获取手机用户: {:?}", user),
        Err(e) => println!("✗ 获取手机用户失败: {}", e),
    }

    // 4. 获取邮箱用户信息
    let get_email_params = GetEmailUserParams::new("user@example.com");
    match user_api.get_email_user(get_email_params) {
        Ok(user) => println!("✓ 获取邮箱用户: {:?}", user),
        Err(e) => println!("✗ 获取邮箱用户失败: {}", e),
    }

    // 5. 同步用户列表
    match user_api.sync_user_list(0) {
        Ok(users) => {
            println!("✓ 同步用户列表: {} 个用户", users.len());
            if let Some(first) = users.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步用户列表失败: {}", e),
    }

    // ============== 账户管理 (Account API) ==============
    println!("\n{}", "=".repeat(40));
    println!("账户管理 (Account API)");
    println!("{}", "=".repeat(40));

    let account_api = client.get_account_api();

    // 1. 获取用户账户余额
    let account_params = GetUserAccountParams::new(uid, "APTOS");
    match account_api.get_user_account(account_params) {
        Ok(account) => {
            println!("✓ 用户账户余额:");
            println!("  币种: {:?}", account.symbol);
            println!("  可用: {:?}", account.balance);
            println!("  冻结: {:?}", account.frozen);
        }
        Err(e) => println!("✗ 获取账户余额失败: {}", e),
    }

    // 2. 获取用户充值地址
    let address_params = GetUserAddressParams::new(uid, "APTOS");
    match account_api.get_user_address(address_params) {
        Ok(address) => {
            println!("✓ 用户充值地址:");
            println!("  地址: {:?}", address.address);
        }
        Err(e) => println!("✗ 获取充值地址失败: {}", e),
    }

    // 3. 根据地址获取用户信息
    let addr_info_params =
        GetUserAddressInfoParams::new("0xd4036730fd450237b8fea382bd887c4c96a8453a");
    match account_api.get_user_address_info(addr_info_params) {
        Ok(info) => println!("✓ 地址用户信息: uid={:?}", info.uid),
        Err(e) => println!("✗ 获取地址用户信息失败: {}", e),
    }

    // 4. 获取公司账户余额
    let company_params = GetCompanyAccountParams::new("APTOS");
    match account_api.get_company_account(company_params) {
        Ok(account) => {
            println!("✓ 公司账户余额:");
            println!("  币种: {:?}", account.symbol);
            println!("  可用: {:?}", account.balance);
            println!("  冻结: {:?}", account.frozen);
        }
        Err(e) => println!("✗ 获取公司账户余额失败: {}", e),
    }

    // 5. 同步用户地址列表
    match account_api.sync_user_address_list(0) {
        Ok(addresses) => {
            println!("✓ 同步地址列表: {} 个地址", addresses.len());
            if let Some(first) = addresses.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步地址列表失败: {}", e),
    }

    // ============== 币种管理 (Coin API) ==============
    println!("\n{}", "=".repeat(40));
    println!("币种管理 (Coin API)");
    println!("{}", "=".repeat(40));

    let coin_api = client.get_coin_api();

    // 获取支持的币种列表
    match coin_api.get_coin_list() {
        Ok(coins) => {
            println!("✓ 支持币种: {} 个", coins.len());
            for coin in coins.iter().take(5) {
                println!("  - {:?}: {:?}", coin.symbol, coin.real_symbol);
            }
            if coins.len() > 5 {
                println!("  ... 还有 {} 个币种", coins.len() - 5);
            }
            if let Some(first) = coins.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 获取币种列表失败: {}", e),
    }

    // ============== 账单管理 (Billing API) ==============
    println!("\n{}", "=".repeat(40));
    println!("账单管理 (Billing API)");
    println!("{}", "=".repeat(40));

    let billing_api = client.get_billing_api();

    // 1. 发起提现
    let withdraw_params = WithdrawParams::new(
        "123456789026",
        uid,
        "0x0f1dc222af5ea2660ff84ae91adc48f1cb2d4991f1e6569dd24d94599c335a06",
        "0.001",
        "APTOS",
    );

    match billing_api.withdraw(withdraw_params) {
        Ok(result) => {
            println!("✓ 提现结果:");
            println!("  ID: {:?}", result.id);
            println!("  request_id ID: {:?}", result.request_id);
        }
        Err(e) => println!("✗ 提现失败: {}", e),
    }

    // 2. 查询提现记录 (按 request_id)
    match billing_api.withdraw_list(&["123456789021", "123456789022"]) {
        Ok(records) => {
            println!("✓ 查询提现记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询提现记录失败: {}", e),
    }

    // 3. 同步提现记录
    match billing_api.sync_withdraw_list(0) {
        Ok(records) => {
            println!("✓ 同步提现记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步提现记录失败: {}", e),
    }

    // 4. 查询充值记录 (按 WaaS ID)
    match billing_api.deposit_list(&["123", "456"]) {
        Ok(records) => {
            println!("✓ 查询充值记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询充值记录失败: {}", e),
    }

    // 5. 同步充值记录
    match billing_api.sync_deposit_list(0) {
        Ok(records) => {
            println!("✓ 同步充值记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步充值记录失败: {}", e),
    }

    // 6. 查询矿工费记录 (按 WaaS ID)
    match billing_api.miner_fee_list(&["123", "456"]) {
        Ok(records) => {
            println!("✓ 查询矿工费记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询矿工费记录失败: {}", e),
    }

    // 7. 同步矿工费记录
    match billing_api.sync_miner_fee_list(0) {
        Ok(records) => {
            println!("✓ 同步矿工费记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步矿工费记录失败: {}", e),
    }

    // ============== 转账管理 (Transfer API) ==============
    println!("\n{}", "=".repeat(40));
    println!("转账管理 (Transfer API)");
    println!("{}", "=".repeat(40));

    let transfer_api = client.get_transfer_api();

    // 1. 内部转账
    let transfer_params = AccountTransferParams::new(
        "transfer_001",
        "USDT",
        "100.5",
        "12345", // from uid
        "67890", // to uid
    )
    .with_remark("Test transfer");

    match transfer_api.account_transfer(transfer_params) {
        Ok(result) => {
            println!("✓ 转账结果:");
            println!("  ID: {:?}", result.id);
            println!("  Request ID: {:?}", result.request_id);
            println!("  状态: {:?}", result.status);
        }
        Err(e) => println!("✗ 转账失败: {}", e),
    }

    // 2. 按 request_id 查询转账记录
    let query_params = GetAccountTransferListParams::by_request_id(&["123", "456"]);
    match transfer_api.get_account_transfer_list(query_params) {
        Ok(records) => println!("✓ 按 request_id 查询转账记录: {} 条", records.len()),
        Err(e) => println!("✗ 查询转账记录失败: {}", e),
    }

    // 3. 按 receipt 查询转账记录
    let query_by_receipt = GetAccountTransferListParams::new("123,456", QueryIdType::Receipt);
    match transfer_api.get_account_transfer_list(query_by_receipt) {
        Ok(records) => println!("✓ 按 receipt 查询转账记录: {} 条", records.len()),
        Err(e) => println!("✗ 查询转账记录失败: {}", e),
    }

    // 4. 同步转账记录
    match transfer_api.sync_account_transfer_list(0) {
        Ok(records) => {
            println!("✓ 同步转账记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步转账记录失败: {}", e),
    }

    // ============== 异步通知 (Async Notify API) ==============
    println!("\n{}", "=".repeat(40));
    println!("异步通知 (Async Notify API)");
    println!("{}", "=".repeat(40));

    let async_notify_api = client.get_async_notify_api();

    // 异步通知 API 使用说明
    println!("异步通知 API 使用说明:");
    println!("  1. notify_request(cipher) - 解密充值/提现通知");
    println!("  2. verify_request(cipher) - 解密提现二次验证请求，返回 WithdrawParams");
    println!("  3. verify_response(params) - 加密提现验证响应，参数为 WithdrawParams");

    // ========== 测试 notify_request ==========
    println!("\n--- notify_request 测试 ---");
    println!("  说明: notify_request 用于解密来自 ChainUp 服务器的回调通知");

    // 使用实际从 ChainUp 服务器收到的加密通知数据
    let encrypted_notify = "jhoA9MtGotqWxqEtB27SwCtJCo9JSIxh2B6m8CItrPQj2gsm6rw-ti1qY5tNP52qXg60FLK49cFj-a84m-57z8aT-Vo-YyJPTcM8Qpuyjj5Pf8tAcbBjBHganULYNPjCCkzgH5n5dlMZIp0tmpc7nV7Pp6hi63KjGGNTfAAbWp7QOVukAsQeQyBFPeKhlVEhq8xqQEN2yg_T1jHRUjIdlTDn2LG_i2tI0MlDpPg5FHL6cViSVM23WBPhJnAFOOrGhaqq06YtVG2m8_x_pLTyI5ZK61Bv0HnDUuIkDuRqNXyhko0sG9uGuKWJ3maWfUc9bSb0VcWPHeWnYUrcE2M9TVtwTEKdcImqZnvjc12YUh_Oz2a9VNls_XN_gTRbeIiTUGsiXX1Yq6OkCCxrsCgD0AXz0KOX4uphZldXq17ZO7sU21-b1y0rsk0qY6PbKRYpp4hhdeKpEfB2gckhf1rc9h17j0ufri4LqsE4EccGuQD4JcSrT5RLY4QRil4wdIO9ZPmhb-Od3zqT9OYPSvPg0QVCVpw-Tn17WfsZw2xB9gO8uzvGcvz9TfUrI8zKg6b6roTR9xt0m0oqMCyhrjAlU35QUh54MHAWI22A3WJkR4d4KhTOrq-2KuCg7Obi3SCoZmVWb28tztUwN6ttc4PJmM370g_YNCiv5Q6F95QgozYAGpu7Kc8ckcsORixNAUpqTCYaZHmST7bxCXDGPaL45H4zHe6IkU-Tf06rY7DoKeMgjGTz3Pb8hrXRXdSCYz9y0MjwGledXqnLiww0Dn_q-qWgOqQs6NeiLG5IqWKJG2e0buav2l_fH-biflRHjpidaTvFnTMUPf9k9-ygWwiWDzM9OD0X-mNdEI6WNe_27O9CtmUTxlBgRJ2tYyhF32a3flQXaA4m34PPXD_HyxFYRQXfqTt_7uaV7NinsnwN8Ll9ccFdXw8BuANu8j24zvBP0zvUyo9d1ywqn0Cw2wt-vPUWF7sZifTLkdr9O7mcAN08ByaIc1MR5ULI-lUsfi6U";

    println!("  加密数据: {}...", &encrypted_notify[..60]);

    // 使用 notify_request 解密实际通知数据
    match async_notify_api.notify_request(encrypted_notify) {
        Ok(notify_data) => {
            println!("  ✓ notify_request 解密成功:");
            println!("    side: {:?}", notify_data.side);
            println!("    id: {:?}", notify_data.id);
            println!("    uid: {:?}", notify_data.uid);
            println!("    email: {:?}", notify_data.email);
            println!("    symbol: {:?}", notify_data.symbol);
            println!("    base_symbol: {:?}", notify_data.base_symbol);
            println!("    amount: {:?}", notify_data.amount);
            println!("    address_to: {:?}", notify_data.address_to);
            println!("    address_from: {:?}", notify_data.address_from);
            println!("    txid: {:?}", notify_data.txid);
            println!("    txid_type: {:?}", notify_data.txid_type);
            println!("    confirmations: {:?}", notify_data.confirmations);
            println!("    contract_address: {:?}", notify_data.contract_address);
            println!("    status: {:?}", notify_data.status);
            println!("    saas_status: {:?}", notify_data.saas_status);
            println!("    company_status: {:?}", notify_data.company_status);
            println!("    request_id: {:?}", notify_data.request_id);
            println!("    withdraw_fee: {:?}", notify_data.withdraw_fee);
            println!(
                "    withdraw_fee_symbol: {:?}",
                notify_data.withdraw_fee_symbol
            );
            println!("    fee: {:?}", notify_data.fee);
            println!("    fee_symbol: {:?}", notify_data.fee_symbol);
            println!("    real_fee: {:?}", notify_data.real_fee);
            println!("    is_mining: {:?}", notify_data.is_mining);
            println!("    created_at: {:?}", notify_data.created_at);
            println!("    updated_at: {:?}", notify_data.updated_at);
        }
        Err(e) => println!("  ✗ notify_request 解密失败: {}", e),
    }

    // ========== 测试 verify_request / verify_response ==========
    println!("\n--- verify_request / verify_response 测试 ---");
    println!("  说明: verify_response 用于加密返回给 ChainUp 的验证结果");
    println!("  说明: verify_request 用于解密来自 ChainUp 的二次验证请求");

    // 创建一个完整的 WithdrawParams
    let sample_withdraw = WithdrawParams::new(
        "verify_test_001",
        uid,
        "0x1234567890abcdef1234567890abcdef12345678",
        "0.01",
        "ETH",
    )
    .with_check_sum("abc123");

    println!("\n  原始提现参数 (WithdrawParams):");
    println!("    request_id: {}", sample_withdraw.request_id);
    println!("    from_uid: {}", sample_withdraw.from_uid);
    println!("    to_address: {}", sample_withdraw.to_address);
    println!("    amount: {}", sample_withdraw.amount);
    println!("    symbol: {}", sample_withdraw.symbol);
    println!("    check_sum: {:?}", sample_withdraw.check_sum);

    // 测试 JSON 序列化/反序列化完整性
    let withdraw_json = serde_json::to_string(&sample_withdraw).unwrap();
    println!("\n  序列化 JSON: {}", withdraw_json);

    match serde_json::from_str::<WithdrawParams>(&withdraw_json) {
        Ok(parsed) => {
            let all_match = sample_withdraw.request_id == parsed.request_id
                && sample_withdraw.from_uid == parsed.from_uid
                && sample_withdraw.to_address == parsed.to_address
                && sample_withdraw.amount == parsed.amount
                && sample_withdraw.symbol == parsed.symbol
                && sample_withdraw.check_sum == parsed.check_sum;

            if all_match {
                println!("  ✓ WithdrawParams 序列化/反序列化验证通过！");
            } else {
                println!("  ✗ WithdrawParams 序列化/反序列化验证失败");
            }
        }
        Err(e) => println!("  ✗ WithdrawParams 反序列化失败: {}", e),
    }

    // 使用 verify_response 加密
    match async_notify_api.verify_response(sample_withdraw.clone()) {
        Ok(encrypted) => {
            println!(
                "\n  ✓ verify_response 加密成功: {}...",
                &encrypted[..50.min(encrypted.len())]
            );
            println!("    (此加密数据可发送给 ChainUp 服务器)");
        }
        Err(e) => println!("  ✗ verify_response 加密失败: {}", e),
    }

    println!("\n{}", "=".repeat(60));
    println!("✓ WaaS 示例完成");
    println!("{}", "=".repeat(60));

    Ok(())
}
