use {
    clap::{crate_description, crate_name, crate_version, Arg, Command},
    futures_util::StreamExt,
    solana_clap_v3_utils::{
        input_parsers::{
            parse_url_or_moniker,
            signer::{SignerSource, SignerSourceParserBuilder},
        },
        input_validators::normalize_to_url_if_moniker,
        keypair::signer_from_path,
    },
    solana_client::{
        nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
        rpc_config::{RpcTransactionLogsConfig, RpcTransactionLogsFilter},
    },
    solana_remote_wallet::remote_wallet::RemoteWalletManager,
    solana_sdk::{
        commitment_config::CommitmentConfig,
        instruction::{AccountMeta, Instruction},
        message::Message,
        pubkey::Pubkey,
        signature::{Keypair, Signature, Signer},
        system_instruction,
        transaction::Transaction,
    },
    std::{process::exit, rc::Rc},
};

struct Config {
    commitment_config: CommitmentConfig,
    default_signer: Box<dyn Signer>,
    json_rpc_url: String,
    verbose: bool,
    websocket_url: String,
}

async fn process_ping(
    rpc_client: &RpcClient,
    signer: &dyn Signer,
    program_id: &Pubkey,
    dry_run: bool,
) -> Result<Signature, Box<dyn std::error::Error>> {
    let new_account = Keypair::new();
    let space = 33;
    let lamports = rpc_client
        .get_minimum_balance_for_rent_exemption(space.try_into()?)
        .await?;
    let mut transaction = Transaction::new_unsigned(Message::new(
        &[
            system_instruction::create_account(
                &signer.pubkey(),
                &new_account.pubkey(),
                lamports,
                space,
                program_id,
            ),
            Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta::new(new_account.pubkey(), false)],
                data: vec![0, 2],
            },
            Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta::new(new_account.pubkey(), false)],
                data: vec![1],
            },
            Instruction {
                program_id: *program_id,
                accounts: vec![AccountMeta::new(new_account.pubkey(), false)],
                data: vec![2, 255, 255, 255, 255],
            },
        ],
        Some(&signer.pubkey()),
    ));

    let blockhash = rpc_client
        .get_latest_blockhash()
        .await
        .map_err(|err| format!("error: unable to get latest blockhash: {err}"))?;

    transaction
        .try_sign(&vec![signer, &new_account], blockhash)
        .map_err(|err| format!("error: failed to sign transaction: {err}"))?;

    if dry_run {
        let result = rpc_client.simulate_transaction(&transaction).await.unwrap();
        println!("{result:?}");
        Ok(transaction.signatures[0])
    } else {
        let signature = rpc_client
            .send_and_confirm_transaction_with_spinner(&transaction)
            .await
            .map_err(|err| format!("error: send transaction: {err}"))?;
        Ok(signature)
    }
}

async fn process_logs(websocket_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let pubsub_client = PubsubClient::new(websocket_url).await?;

    let (mut logs, logs_unsubscribe) = pubsub_client
        .logs_subscribe(
            RpcTransactionLogsFilter::All,
            RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::confirmed()),
            },
        )
        .await?;

    while let Some(log) = logs.next().await {
        println!("Transaction executed in slot {}:", log.context.slot);
        println!("  Signature: {}:", log.value.signature);
        println!(
            "  Status: {}",
            log.value
                .err
                .map(|err| err.to_string())
                .unwrap_or_else(|| "Success".into())
        );
        println!("  Log Messages:");
        for msg in log.value.logs {
            println!("    {msg}");
        }
    }
    logs_unsubscribe().await;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_matches = Command::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .subcommand_required(true)
        .arg_required_else_help(true)
        .arg({
            let arg = Arg::new("config_file")
                .short('C')
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");
            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::new("keypair")
                .long("keypair")
                .value_name("KEYPAIR")
                .value_parser(SignerSourceParserBuilder::default().allow_all().build())
                .takes_value(true)
                .global(true)
                .help("Filepath or URL to a keypair [default: client keypair]"),
        )
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::new("json_rpc_url")
                .short('u')
                .long("url")
                .value_name("URL")
                .takes_value(true)
                .global(true)
                .value_parser(parse_url_or_moniker)
                .help("JSON RPC URL for the cluster [default: value from configuration file]"),
        )
        .subcommand(
            Command::new("ping")
                .about("Send a ping transaction")
                .arg(
                    Arg::new("address")
                        .value_parser(SignerSourceParserBuilder::default().allow_all().build())
                        .value_name("ADDRESS")
                        .takes_value(true)
                        .index(1)
                        .help("Address of the helloworld program to ping"),
                )
                .arg(
                    Arg::new("dry_run")
                        .long("dry-run")
                        .help("Simulate the transaction"),
                ),
        )
        .subcommand(Command::new("logs").about("Stream transaction logs"))
        .get_matches();

    let (command, matches) = app_matches.subcommand().unwrap();
    let mut wallet_manager: Option<Rc<RemoteWalletManager>> = None;

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        let default_signer = if let Ok(Some((signer, _))) =
            SignerSource::try_get_signer(matches, "keypair", &mut wallet_manager)
        {
            Box::new(signer)
        } else {
            signer_from_path(
                matches,
                &cli_config.keypair_path,
                "keypair",
                &mut wallet_manager,
            )?
        };

        let json_rpc_url = normalize_to_url_if_moniker(
            matches
                .value_of("json_rpc_url")
                .unwrap_or(&cli_config.json_rpc_url),
        );

        let websocket_url = solana_cli_config::Config::compute_websocket_url(&json_rpc_url);
        Config {
            commitment_config: CommitmentConfig::confirmed(),
            default_signer,
            json_rpc_url,
            verbose: matches.is_present("verbose"),
            websocket_url,
        }
    };
    solana_logger::setup_with_default("solana=info");

    if config.verbose {
        println!("JSON RPC URL: {}", config.json_rpc_url);
        println!("Websocket URL: {}", config.websocket_url);
    }
    let rpc_client =
        RpcClient::new_with_commitment(config.json_rpc_url.clone(), config.commitment_config);

    match (command, matches) {
        ("logs", _arg_matches) => {
            process_logs(&config.websocket_url)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("error: {err}");
                    exit(1);
                });
        }
        ("ping", arg_matches) => {
            let address =
                SignerSource::try_get_pubkey(arg_matches, "address", &mut wallet_manager)?.unwrap();
            let dry_run = arg_matches.is_present("dry_run");
            let signature = process_ping(
                &rpc_client,
                config.default_signer.as_ref(),
                &address,
                dry_run,
            )
            .await
            .unwrap_or_else(|err| {
                eprintln!("error: send transaction: {err}");
                exit(1);
            });
            println!("Signature: {signature}");
        }
        _ => unreachable!(),
    };

    Ok(())
}

#[cfg(test)]
mod test {
    use {
        super::*,
        solana_sdk::bpf_loader_upgradeable,
        solana_test_validator::*,
        std::{env, path::PathBuf},
    };

    #[tokio::test]
    async fn test_ping() {
        let mut test_validator_genesis = TestValidatorGenesis::default();
        let sbf_out_dir = env::var("SBF_OUT_DIR").unwrap();
        let mut program_path = PathBuf::from(sbf_out_dir);
        program_path.push("helloworld.so");
        let program_id = Pubkey::new_unique();
        test_validator_genesis.add_upgradeable_programs_with_path(&[UpgradeableProgramInfo {
            program_id,
            loader: bpf_loader_upgradeable::id(),
            program_path,
            upgrade_authority: Pubkey::new_unique(),
        }]);
        let (test_validator, payer) = test_validator_genesis.start_async().await;
        let rpc_client = test_validator.get_async_rpc_client();

        assert!(matches!(
            process_ping(&rpc_client, &payer, &program_id, false).await,
            Ok(_)
        ));
    }
}
