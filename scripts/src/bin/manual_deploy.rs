#![cfg(not(test))]
use clap::Parser;
use cw_orch::{daemon::DaemonBuilder, prelude::*};
use terp_account::DEPLOYMENT_DAO;
use terp_account_scripts::{
    networks::{ping_grpc, LOCAL_TERP, TERP_MAINNET, TERP_TESTNET},
    *,
};
use tokio::runtime::Runtime;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Network to deploy on: main, testnet, local
    #[clap(short, long, default_value = "main")]
    network: String,

    #[clap(short, long, default_value = "deploy_on")]
    method: String,
}

fn main() {
    let args = Args::parse();
    env_logger::init();
    dotenv::dotenv().ok();

    println!("Deploying Terp Accounts Framework...");

    let (terp_chain, is_local) = match args.network.as_str() {
        "main" => (TERP_MAINNET.to_owned(), false),
        "testnet" => (TERP_TESTNET.to_owned(), false),
        "local" => (LOCAL_TERP.to_owned(), true),
        _ => panic!("Invalid network: use main, testnet, or local"),
    };

    if let Err(ref err) = manual_deploy(terp_chain.into(), args.method, is_local) {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        ::std::process::exit(1);
    }
}

fn manual_deploy(network: ChainInfoOwned, _method: String, is_local: bool) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let urls = network.grpc_urls.to_vec();
    for url in urls {
        rt.block_on(ping_grpc(&url))?;
    }

    let mut chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;

    // For local network, use sender as admin (no AuthZ DAO)
    // For mainnet/testnet, use the DEPLOYMENT_DAO as AuthZ granter
    let admin = if is_local {
        chain.sender_addr()
    } else {
        let le_granter = Addr::unchecked(DEPLOYMENT_DAO.to_string());
        println!("Using AuthZ granter: {}", le_granter);
        chain.sender_mut().set_authz_granter(&le_granter);
        le_granter
    };

    let suite = TerpAccountSuite::deploy_on(chain.clone(), admin)?;

    // Print addresses for shell script consumption
    println!("CONTRACT_ADDR:terp721_account={}", suite.nft.addr_str()?);
    println!(
        "CONTRACT_ADDR:terp721_account_manifold={}",
        suite.manifold.addr_str()?
    );

    Ok(())
}
