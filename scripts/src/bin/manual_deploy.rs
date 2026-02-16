#![cfg(not(test))]
use clap::Parser;
use cw_orch::{daemon::DaemonBuilder, prelude::*};
use terp_account::DEPLOYMENT_DAO;
use terp_account_scripts::{
    networks::{ping_grpc, TERP_MAINNET, terp_TESTNET},
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
    // parse cargo command arguments for network type
    let args = Args::parse();
    // logs any errors
    env_logger::init();
    dotenv::dotenv().ok();

    println!("Deploying Terp Accounts Framework...");

    let terp_chain = match args.network.as_str() {
        "main" => TERP_MAINNET.to_owned(),
        "testnet" => terp_TESTNET.to_owned(),
        // "local" => LOCAL_NETWORK1.to_owned(),
        _ => panic!("Invalid network"),
    };

    if let Err(ref err) = manual_deploy(terp_chain.into(), args.method) {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        ::std::process::exit(1);
    }
}

fn manual_deploy(network: ChainInfoOwned, _method: String) -> anyhow::Result<()> {
    let rt = Runtime::new()?;
    let urls = network.grpc_urls.to_vec();
    for url in urls {
        rt.block_on(ping_grpc(&url))?;
    }

    let mut chain = DaemonBuilder::new(network.clone())
        .handle(rt.handle())
        .build()?;

    let le_granter = &Addr::unchecked(DEPLOYMENT_DAO.to_string());
    println!("Using AuthZ granter: {}", le_granter);
    chain.sender_mut().set_authz_granter(le_granter);

    let _terp = TerpAccountSuite::deploy_on(chain.clone(), le_granter.clone())?;

    Ok(())
}

// TODO:
// fn standard_mint() {}
// fn select_bids() {}
// fn load_from_state() {}
