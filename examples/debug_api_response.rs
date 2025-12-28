//! 调试 MPC API 返回数据结构
//!
//! 使用方法:
//!   cargo run --example debug_api_response              # 显示帮助信息
//!   cargo run --example debug_api_response all          # 运行所有 API 测试
//!   cargo run --example debug_api_response supported    # 测试获取支持的币种
//!   cargo run --example debug_api_response coin_list    # 测试币种详情
//!   cargo run --example debug_api_response block_height # 测试最新区块高度
//!   ... 等等

use chainup_custody_sdk::{error::ChainUpError, mpc::MpcClient};
use std::env;

const APP_ID: &str = "";
const RSA_PRIVATE_KEY: &str = r#""#;
const WAAS_PUBLIC_KEY: &str = r#""#;
const SIGN_PRIVATE_KEY: &str = r#""#;

const SUB_WALLET_ID: i64 = 1000537;

fn create_client() -> Result<MpcClient, ChainUpError> {
    MpcClient::builder()
        .set_app_id(APP_ID)
        .set_rsa_private_key(RSA_PRIVATE_KEY)
        .set_waas_public_key(WAAS_PUBLIC_KEY)
        .set_sign_private_key(SIGN_PRIVATE_KEY)
        .set_debug(true)
        .build()
}

fn print_usage() {
    println!("MPC API 调试工具");
    println!("{}", "=".repeat(60));
    println!("\n使用方法: cargo run --example debug_api_response <command>");
    println!("\n可用命令:");
    println!("  all              - 运行所有 API 测试");
    println!("");
    println!("  === Wallet API ===");
    println!("  address_info     - 地址信息 (/api/mpc/sub_wallet/address/info)");
    println!("  query_address    - 查询钱包地址 (/api/mpc/sub_wallet/get/address/list)");
    println!("  wallet_assets    - 获取钱包资产 (/api/mpc/sub_wallet/assets)");
    println!("  change_status    - 修改钱包显示状态 (/api/mpc/sub_wallet/show_status)");
    println!("");
    println!("  === Workspace API ===");
    println!("  supported        - 支持的币种 (/api/mpc/coin/supported)");
    println!("  coin_list        - 币种详情 (/api/mpc/coin/list)");
    println!("  block_height     - 最新区块高度 (/api/mpc/main/coin/last/block/height)");
    println!("");
    println!("  === Deposit API ===");
    println!("  deposit          - 同步充值记录 (/api/mpc/trans/sync/deposit)");
    println!("");
    println!("  === Withdraw API ===");
    println!("  withdraw         - 同步提现记录 (/api/mpc/trans/sync/withdraw)");
    println!("");
    println!("  === Auto Sweep API ===");
    println!("  auto_collect     - 同步自动归集记录 (/api/mpc/auto_collect/sync/list)");
    println!("  auto_sub_wallets - 自动归集钱包列表 (/api/mpc/auto_collect/sub_wallets)");
    println!("");
    println!("  === Web3 API ===");
    println!("  web3             - 同步 Web3 交易记录 (/api/mpc/web3/trans/sync/list)");
    println!("");
    println!("  === TRON Resource API ===");
    println!("  tron_resource    - 同步 TRON 资源购买记录 (/api/mpc/tron/buy_resource/sync/list)");
}

// ==================== Wallet API ====================

/// 测试地址信息接口
fn test_address_info() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("地址信息 - /api/mpc/sub_wallet/address/info");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::WalletAddressInfoParams;

    match client
        .get_wallet_api()
        .wallet_address_info(WalletAddressInfoParams::new(
            "0x633A84Ee0ab29d911e5466e5E1CB9cdBf5917E72",
        )) {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试查询钱包地址接口
fn test_query_address() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("查询钱包地址 - /api/mpc/sub_wallet/get/address/list");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::QueryWalletAddressParams;

    match client
        .get_wallet_api()
        .query_wallet_address(QueryWalletAddressParams::new(SUB_WALLET_ID, "ETH"))
    {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试获取钱包资产接口
fn test_wallet_assets() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("获取钱包资产 - /api/mpc/sub_wallet/assets");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::GetWalletAssetsParams;

    match client
        .get_wallet_api()
        .get_wallet_assets(GetWalletAssetsParams::new(SUB_WALLET_ID, "ETH"))
    {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试修改钱包显示状态接口
fn test_change_wallet_show_status() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("修改钱包显示状态 - /api/mpc/sub_wallet/show_status");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::ChangeWalletShowStatusParams;

    match client
        .get_wallet_api()
        .change_wallet_show_status(ChangeWalletShowStatusParams::from_ids(&[SUB_WALLET_ID], 1))
    {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Workspace API ====================

/// 测试支持的币种接口
fn test_supported_coins() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("支持的币种 - /api/mpc/coin/supported");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_workspace_api().get_supported_coins() {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试币种详情接口
fn test_coin_details() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("币种详情 - /api/mpc/coin/list");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::GetCoinDetailsParams;

    match client
        .get_workspace_api()
        .get_coin_details(GetCoinDetailsParams::new("ETH"))
    {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试最新区块高度接口
fn test_block_height() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("最新区块高度 - /api/mpc/main/coin/last/block/height");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::GetLastBlockHeightParams;

    match client
        .get_workspace_api()
        .get_last_block_height(GetLastBlockHeightParams::new("ETH"))
    {
        Ok(r) => println!("结果: {:?}", r),
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Deposit API ====================

/// 测试同步充值记录接口
fn test_deposit_records() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("同步充值记录 - /api/mpc/trans/sync/deposit");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_deposit_api().sync_deposit_records(0) {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Withdraw API ====================

/// 测试同步提现记录接口
fn test_withdraw_records() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("同步提现记录 - /api/mpc/trans/sync/withdraw");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_withdraw_api().sync_withdraw_records(0) {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Auto Sweep API ====================

/// 测试同步自动归集记录接口
fn test_auto_collect_records() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("同步自动归集记录 - /api/mpc/auto_collect/sync/list");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_auto_sweep_api().sync_auto_collect_records(0) {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

/// 测试自动归集钱包列表接口
fn test_auto_sub_wallets() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("自动归集钱包列表 - /api/mpc/auto_collect/sub_wallets");
    println!("{}", "=".repeat(60));

    let client = create_client()?;
    use chainup_custody_sdk::mpc_types::AutoCollectSubWalletsParams;

    match client
        .get_auto_sweep_api()
        .auto_collect_sub_wallets(AutoCollectSubWalletsParams::new("USDTERC20"))
    {
        Ok(r) => {
            println!("结果: {:?}", r);
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Web3 API ====================

/// 测试同步 Web3 交易记录接口
fn test_web3_trans_records() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("同步 Web3 交易记录 - /api/mpc/web3/trans/sync/list");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_web3_api().sync_web3_trans_records(0) {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== TRON Resource API ====================

/// 测试同步 TRON 资源购买记录接口
fn test_tron_resource_records() -> Result<(), ChainUpError> {
    println!("\n{}", "=".repeat(60));
    println!("同步 TRON 资源购买记录 - /api/mpc/tron/buy_resource/sync/list");
    println!("{}", "=".repeat(60));

    let client = create_client()?;

    match client.get_tron_resource_api().sync_buy_resource_records(0) {
        Ok(r) => {
            println!("结果: {} 条", r.len());
            if let Some(f) = r.first() {
                println!("首条: {:?}", f);
            }
        }
        Err(e) => println!("错误: {}", e),
    }
    Ok(())
}

// ==================== Main ====================

fn run_all() -> Result<(), ChainUpError> {
    test_address_info()?;
    test_query_address()?;
    test_wallet_assets()?;
    test_supported_coins()?;
    test_coin_details()?;
    test_block_height()?;
    test_deposit_records()?;
    test_withdraw_records()?;
    test_auto_collect_records()?;
    test_web3_trans_records()?;
    test_tron_resource_records()?;
    Ok(())
}

fn main() -> Result<(), ChainUpError> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    let command = args[1].as_str();

    match command {
        "all" => run_all()?,
        "address_info" => test_address_info()?,
        "query_address" => test_query_address()?,
        "wallet_assets" => test_wallet_assets()?,
        "change_status" => test_change_wallet_show_status()?,
        "supported" => test_supported_coins()?,
        "coin_list" => test_coin_details()?,
        "block_height" => test_block_height()?,
        "deposit" => test_deposit_records()?,
        "withdraw" => test_withdraw_records()?,
        "auto_collect" => test_auto_collect_records()?,
        "auto_sub_wallets" => test_auto_sub_wallets()?,
        "web3" => test_web3_trans_records()?,
        "tron_resource" => test_tron_resource_records()?,
        _ => {
            println!("未知命令: {}", command);
            print_usage();
        }
    }

    Ok(())
}
