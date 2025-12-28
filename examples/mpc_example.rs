//! MPC API 使用示例
//!
//! 运行命令: cargo run --example mpc_example

use chainup_custody_sdk::{
    error::ChainUpError,
    mpc::MpcClient,
    mpc_types::{
        AccelerateWeb3TransParams,
        // Auto Sweep API
        AutoCollectSubWalletsParams,
        ChangeWalletShowStatusParams,
        // Tron Resource API
        CreateTronDelegateParams,
        CreateWalletAddressParams,
        // Wallet API
        CreateWalletParams,
        // Web3 API
        CreateWeb3TransParams,
        // Workspace API
        GetCoinDetailsParams,
        // Deposit API
        GetDepositRecordsParams,
        GetLastBlockHeightParams,
        GetWalletAssetsParams,
        // Notify API
        QueryWalletAddressParams,
        SetAutoCollectSymbolParams,
        WalletAddressInfoParams,
        // Withdraw API
        WithdrawParams,
    },
};

fn main() -> Result<(), ChainUpError> {
    println!("{}", "=".repeat(60));
    println!("ChainUp Custody Rust SDK - MPC 示例");
    println!("{}", "=".repeat(60));

    // ============== 配置客户端 ==============
    // 请替换为你的实际配置
    let app_id = "";
    let rsa_private_key = r#""#;
    let waas_public_key = r#""#;
    let sign_private_key = r#""#;

    // 使用 Builder 模式创建客户端
    let client = MpcClient::builder()
        .set_app_id(app_id)
        .set_rsa_private_key(rsa_private_key)
        .set_waas_public_key(waas_public_key)
        .set_sign_private_key(sign_private_key)
        .set_debug(false)
        .build()?;

    println!("\n✓ MPC 客户端创建成功");

    let sub_wallet_id = 1000537_i64; // 替换为实际钱包ID

    // ============== 钱包管理 (Wallet API) ==============
    println!("\n{}", "=".repeat(40));
    println!("钱包管理 (Wallet API)");
    println!("{}", "=".repeat(40));

    let wallet_api = client.get_wallet_api();

    // 1. 创建钱包
    let wallet_params = CreateWalletParams::new("TestWallet32").with_show_status(1);
    match wallet_api.create_wallet(wallet_params) {
        Ok(wallet) => println!("✓ 创建钱包: {:?}", wallet),
        Err(e) => println!("✗ 创建钱包失败: {}", e),
    }

    // 2. 创建钱包地址
    let address_params = CreateWalletAddressParams::new(sub_wallet_id, "ETH");
    match wallet_api.create_wallet_address(address_params) {
        Ok(address) => println!("✓ 创建地址: {:?}", address),
        Err(e) => println!("✗ 创建地址失败: {}", e),
    }

    // 3. 查询钱包地址列表
    let query_addr_params = QueryWalletAddressParams::new(sub_wallet_id, "ETH").with_max_id(0);
    match wallet_api.query_wallet_address(query_addr_params) {
        Ok(addresses) => {
            println!("✓ 地址列表: {} 个地址", addresses.len());
            if let Some(first) = addresses.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询地址失败: {}", e),
    }

    // 4. 获取钱包资产
    let assets_params = GetWalletAssetsParams::new(sub_wallet_id, "BSC");
    match wallet_api.get_wallet_assets(assets_params) {
        Ok(assets) => println!("✓ 钱包资产: {:?}", assets),
        Err(e) => println!("✗ 查询资产失败: {}", e),
    }

    // 5. 修改钱包显示状态
    let status_params = ChangeWalletShowStatusParams::from_ids(&[sub_wallet_id], 1);
    match wallet_api.change_wallet_show_status(status_params) {
        Ok(result) => println!("✓ 修改显示状态: {}", result),
        Err(e) => println!("✗ 修改显示状态失败: {}", e),
    }

    // 6. 查询地址信息
    let addr_info_params =
        WalletAddressInfoParams::new("0x633A84Ee0ab29d911e5466e5E1CB9cdBf5917E72");
    match wallet_api.wallet_address_info(addr_info_params) {
        Ok(info) => {
            println!("✓ 地址信息: {:?}", info);
        }
        Err(e) => println!("✗ 查询地址信息失败: {}", e),
    }

    // ============== 充值管理 (Deposit API) ==============
    println!("\n{}", "=".repeat(40));
    println!("充值管理 (Deposit API)");
    println!("{}", "=".repeat(40));

    let deposit_api = client.get_deposit_api();

    // 1. 同步充值记录
    match deposit_api.sync_deposit_records(0) {
        Ok(records) => {
            println!("✓ 同步充值记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }

            // 2. 获取特定充值记录
            if !records.is_empty() {
                let deposit_ids: Vec<i64> = records.iter().take(3).filter_map(|d| d.id).collect();
                if !deposit_ids.is_empty() {
                    let params = GetDepositRecordsParams::new(deposit_ids);
                    match deposit_api.get_deposit_records(params) {
                        Ok(recs) => {
                            println!("✓ 获取充值记录: {} 条", recs.len());
                            if let Some(first) = recs.first() {
                                println!("  首条: {:?}", first);
                            }
                        }
                        Err(e) => println!("✗ 获取充值记录失败: {}", e),
                    }
                }
            }
        }
        Err(e) => println!("✗ 同步充值记录失败: {}", e),
    }

    // ============== 提现管理 (Withdraw API) ==============
    println!("\n{}", "=".repeat(40));
    println!("提现管理 (Withdraw API)");
    println!("{}", "=".repeat(40));

    let withdraw_api = client.get_withdraw_api();

    // 1. 发起提现
    let withdraw_params = WithdrawParams::new(
        "123456789029",
        sub_wallet_id,
        "Sepolia",
        "0.001",
        "0xdcb0D867403adE76e75a4A6bBcE9D53C9d05B981",
    )
    .with_remark("Test withdrawal")
    .with_transaction_sign();

    match withdraw_api.withdraw(withdraw_params) {
        Ok(result) => println!("✓ 提现结果: {:?}", result),
        Err(e) => println!("✗ 提现失败: {}", e),
    }

    // 2. 同步提现记录
    match withdraw_api.sync_withdraw_records(0) {
        Ok(records) => {
            println!("✓ 同步提现记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }

            // 3. 获取特定提现记录
            if !records.is_empty() {
                let request_ids: Vec<&str> = records
                    .iter()
                    .take(3)
                    .filter_map(|w| w.request_id.as_deref())
                    .collect();
                if !request_ids.is_empty() {
                    match withdraw_api.get_withdraw_records(&request_ids) {
                        Ok(recs) => {
                            println!("✓ 获取提现记录: {} 条", recs.len());
                            if let Some(first) = recs.first() {
                                println!("  首条: {:?}", first);
                            }
                        }
                        Err(e) => println!("✗ 获取提现记录失败: {}", e),
                    }
                }
            }
        }
        Err(e) => println!("✗ 同步提现记录失败: {}", e),
    }

    // ============== Web3 交易 (Web3 API) ==============
    println!("\n{}", "=".repeat(40));
    println!("Web3 交易 (Web3 API)");
    println!("{}", "=".repeat(40));

    let web3_api = client.get_web3_api();

    // 1. 创建 Web3 交易
    let web3_params = CreateWeb3TransParams::new(
        "web3_12345678",
        sub_wallet_id,
        "ETH",
        "0x1234567890abcdef1234567890abcdef12345678",
        "0",
        "20",    // gas_price
        "21000", // gas_limit
        "0x",    // input_data
        "1",     // trans_type
    )
    .with_dapp_name("Test Dapp")
    .with_dapp_url("https://example.com")
    .with_transaction_sign();

    match web3_api.create_web3_trans(web3_params) {
        Ok(result) => println!("✓ Web3 交易结果: {:?}", result),
        Err(e) => println!("✗ 创建 Web3 交易失败: {}", e),
    }

    // 2. 加速 Web3 交易
    let accelerate_params = AccelerateWeb3TransParams::new(
        12345678, // trans_id: Web3 交易ID
        "25",     // 新 gas_price (Gwei)
        "21000",  // gas_limit
    );
    match web3_api.accelerate_web3_trans(accelerate_params) {
        Ok(result) => println!("✓ 加速交易结果: {:?}", result),
        Err(e) => println!("✗ 加速交易失败: {}", e),
    }

    // 3. 查询 Web3 交易记录
    match web3_api.get_web3_trans_records(&["web3_12345678"]) {
        Ok(records) => {
            println!("✓ Web3 交易记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询 Web3 交易记录失败: {}", e),
    }

    // 4. 同步 Web3 交易记录
    match web3_api.sync_web3_trans_records(0) {
        Ok(records) => {
            println!("✓ 同步 Web3 交易记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步 Web3 交易记录失败: {}", e),
    }

    // ============== 自动归集 (Auto Sweep API) ==============
    println!("\n{}", "=".repeat(40));
    println!("自动归集 (Auto Sweep API)");
    println!("{}", "=".repeat(40));

    let auto_sweep_api = client.get_auto_sweep_api();

    // 1. 同步归集记录
    match auto_sweep_api.sync_auto_collect_records(0) {
        Ok(records) => {
            println!("✓ 同步归集记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步归集记录失败: {}", e),
    }

    // 2. 获取自动归集钱包
    let sweep_params = AutoCollectSubWalletsParams::new("USDTERC20");
    match auto_sweep_api.auto_collect_sub_wallets(sweep_params) {
        Ok(result) => println!("✓ 自动归集钱包: {:?}", result),
        Err(e) => println!("✗ 获取自动归集钱包失败: {}", e),
    }

    // 3. 设置自动归集币种配置
    let symbol_params = SetAutoCollectSymbolParams::new("USDTERC20", "100", "0.01");
    match auto_sweep_api.set_auto_collect_symbol(symbol_params) {
        Ok(()) => println!("✓ 设置归集币种成功"),
        Err(e) => println!("✗ 设置归集币种失败: {}", e),
    }

    // ============== 工作区管理 (Workspace API) ==============
    println!("\n{}", "=".repeat(40));
    println!("工作区管理 (Workspace API)");
    println!("{}", "=".repeat(40));

    let workspace_api = client.get_workspace_api();

    // 1. 获取支持的币种
    match workspace_api.get_supported_coins() {
        Ok(response) => {
            println!("✓ 已开通主链数量: {}", response.open_main_chain.len());
            println!("✓ 支持主链数量: {}", response.support_main_chain.len());
            if let Some(first) = response.open_main_chain.first() {
                println!("  已开通首条: {:?}", first);
            }

            if let Some(first) = response.support_main_chain.first() {
                println!("  支持主链数量: {:?}", first);
            }
        }
        Err(e) => println!("✗ 获取支持币种失败: {}", e),
    }

    // 2. 获取币种详情
    let coin_params = GetCoinDetailsParams::new("ETH");
    match workspace_api.get_coin_details(coin_params) {
        Ok(details) => {
            println!("✓ 币种详情数量: {} 条", details.len());
            if let Some(first) = details.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 获取币种详情失败: {}", e),
    }

    // 3. 获取代币详情（带主链）
    let token_params = GetCoinDetailsParams::with_main_chain("USDTERC20", "ETH");
    match workspace_api.get_coin_details(token_params) {
        Ok(details) => {
            println!("✓ 代币详情数量: {} 条", details.len());
            if let Some(first) = details.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 获取代币详情失败: {}", e),
    }

    // 4. 获取最新区块高度
    let height_params = GetLastBlockHeightParams::new("ETH");
    match workspace_api.get_last_block_height(height_params) {
        Ok(info) => println!("✓ 区块高度: {:?}", info),
        Err(e) => println!("✗ 获取区块高度失败: {}", e),
    }

    // ============== TRON 资源管理 (Tron Resource API) ==============
    println!("\n{}", "=".repeat(40));
    println!("TRON 资源管理 (Tron Resource API)");
    println!("{}", "=".repeat(40));

    let tron_api = client.get_tron_resource_api();

    // 1. 购买 TRON 资源 (能量)
    let tron_params = CreateTronDelegateParams::new(
        "tron_12345678",                      // request_id
        "TPjJg9FnzQuYBd6bshgaq7rkH4s36zju5S", // address_from
        "10010",                              // service_charge_type (10min)
    )
    .with_buy_type(0) // 系统购买
    .with_resource_type(0) // Energy
    .with_energy_num(32000)
    .with_address_to("TGmBzYfBBtMfFF8v9PweTaPwn3WoB7aGPd")
    .with_contract_address("TR7NHqjeKQxGTCi8q8ZY4pL8otSzgjLj6t");

    match tron_api.create_tron_delegate(tron_params) {
        Ok(result) => println!("✓ TRON 资源购买结果: {:?}", result),
        Err(e) => println!("✗ 购买 TRON 资源失败: {}", e),
    }

    // 2. 查询资源购买记录
    match tron_api.get_buy_resource_records(&["tron_12345678"]) {
        Ok(records) => {
            println!("✓ TRON 资源记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 查询 TRON 资源记录失败: {}", e),
    }

    // 3. 同步资源购买记录
    match tron_api.sync_buy_resource_records(0) {
        Ok(records) => {
            println!("✓ 同步 TRON 资源记录: {} 条", records.len());
            if let Some(first) = records.first() {
                println!("  首条: {:?}", first);
            }
        }
        Err(e) => println!("✗ 同步 TRON 资源记录失败: {}", e),
    }

    // ============== 通知处理 (Notify API) ==============
    println!("\n{}", "=".repeat(40));
    println!("通知处理 (Notify API)");
    println!("{}", "=".repeat(40));

    let notify_api = client.get_notify_api();

    // 测试用加密数据（在 webhook 回调中使用）
    let encrypted_data = "Af-uUJj8a2-Og7E5CwzANv4vo8NMf-z-DijwrIuK74Or8eRveM7G_-f0ErtX4WurcVrjdWC-tqU0BDhBwiDijbdyCFBvYB5UmLnHL_Rg13amhQTM-kaHoh-U9WPhYB3vGRwWkTwJ_aETERVVciAvoTf5CalqydMSe8G3KNz-ymrSVUe92DfW5ZdDKJm1hNYYteGJvg0hk--GRiPybPv2W78NlTLyWmXq094megsVzZv-KlsEGPUvPoBnEJ0Xu__AO-l-GfCG4rVO4rb8J01Nq_0Q9eRKcKWq0ci7MfnPPLMhtAWwRvSd3U8PUNHOLqGaJzOLraFnuFUHn90h7T23_DeAduA2W6dto99qb8YQ_iVnMnOKfE0Ls7Vv5S2qhgQJ0nl-BA3PPPOwW37cMb-wTbi3ZezU_S1NQEbrruEChkPhTaK0AqsM6mESV8wGflcWx3N9XPv6QatJ9zedBnkfJ4bJ4Vy2rUEtQF8eVc6zXhV8PuDRiSMf0V0yxzMjE6o9z0s087KSAqFphitlHvQMPJ29FUnyvCe_Czr5WPuhl89GOZjERE2uoNTfHqAlZVzMamoPv4y0qyIjJTufAQm-WwrQK9kGesky7eCiOXVdtR9UhEYpzEJSgXxENjUrHMx6D2AlEzlr17a2DgI-WrWB7oUnyiNnf__ElmLPPkJBdFUfzJByQkLxkUB0FLvTWdVbiIRPmPpdgb7jkhJsHUSOH0NmULqu8bYiEQtGfqRJh8I98qDzHWwfE_VAbqwATj2oD959Fm1eInBqh7eXGoy2WR3o00VpPrNvoE4eJNmw3WpVzlRF7ZVwOpcWRT-dHTShz9mB2Etk9P8D4rGmMZyXHkt4aGUJkE1b3cOEjzkOEFX8CaNe-VHiBYhIyFzMetn7mfIFB0hl565FGEumbhDKNNz_m9T2qPM5k4BQ9fLWUt_WJAVdC81_piIlBOQfYPDbdYoc_9ser1p-Jy5cgTyOMdWuSWC3jMsT09xr8dMcLkKmd39khGidAvGqOOPL1ST0";

    // 使用 decrypt_notification 解密通知数据
    println!("\n--- decrypt_notification ---");
    match notify_api.decrypt_notification(encrypted_data) {
        Ok(data) => {
            println!("✓ 解密成功，字段详情:");
            println!("  id: {:?}", data.id);
            println!("  side: {:?}", data.side);
            println!("  notify_type: {:?}", data.notify_type);
            println!("  request_id: {:?}", data.request_id);
            println!("  sub_wallet_id: {:?}", data.sub_wallet_id);
            println!("  app_id: {:?}", data.app_id);
            println!("  main_chain_symbol: {:?}", data.main_chain_symbol);
            println!("  base_symbol: {:?}", data.base_symbol);
            println!("  symbol: {:?}", data.symbol);
            println!("  contract_address: {:?}", data.contract_address);
            println!("  amount: {:?}", data.amount);
            println!("  fee: {:?}", data.fee);
            println!("  real_fee: {:?}", data.real_fee);
            println!("  fee_symbol: {:?}", data.fee_symbol);
            println!("  refund_amount: {:?}", data.refund_amount);
            println!("  delegate_fee: {:?}", data.delegate_fee);
            println!("  txid: {:?}", data.txid);
            println!("  tx_height: {:?}", data.tx_height);
            println!("  block_height: {:?}", data.block_height);
            println!("  block_time: {:?}", data.block_time);
            println!("  confirmations: {:?}", data.confirmations);
            println!("  from: {:?}", data.from);
            println!("  to: {:?}", data.to);
            println!("  memo: {:?}", data.memo);
            println!("  status: {:?}", data.status);
            println!("  address_from: {:?}", data.address_from);
            println!("  address_to: {:?}", data.address_to);
            println!("  confirm: {:?}", data.confirm);
            println!("  safe_confirm: {:?}", data.safe_confirm);
            println!("  is_mining: {:?}", data.is_mining);
            println!("  trans_type: {:?}", data.trans_type);
            println!("  withdraw_source: {:?}", data.withdraw_source);
            println!("  kyt_status: {:?}", data.kyt_status);
            println!("  interactive_contract: {:?}", data.interactive_contract);
            println!("  input_data: {:?}", data.input_data);
            println!("  dapp_img: {:?}", data.dapp_img);
            println!("  dapp_name: {:?}", data.dapp_name);
            println!("  dapp_url: {:?}", data.dapp_url);
            println!("  charset: {:?}", data.charset);
            println!("  sign: {:?}", data.sign);
            println!("  notify_time: {:?}", data.notify_time);
            println!("  created_at: {:?}", data.created_at);
            println!("  updated_at: {:?}", data.updated_at);
            if !data.extra.is_empty() {
                println!("  extra (未定义字段): {:?}", data.extra);
            }
        }
        Err(e) => println!("✗ decrypt_notification 失败: {}", e),
    }

    println!("\n{}", "=".repeat(60));
    println!("✓ MPC 示例完成");
    println!("{}", "=".repeat(60));

    Ok(())
}
