use sp_core::Pair;
use sp_keyring::AccountKeyring;
use subxt::{tx::PairSigner, OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "metadata.scale")]
pub mod substrate {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let alice_signer = PairSigner::new(AccountKeyring::Alice.pair());

    let (ecdsa_pair, _) = sp_core::ecdsa::Pair::from_phrase(
        "bottom drive obey lake curtain smoke basket hold race lonely fit walk",
        None,
    )
    .unwrap();
    let ecdsa_signer = PairSigner::new(ecdsa_pair);

    // 5GKyBtzbxKU1qjhZrKpMiwtJj7o6jJcXbKQVtYq74DCPerXN
    let ecdsa_addr = ecdsa_signer.account_id();

    let api = OnlineClient::<PolkadotConfig>::new().await?;

    let tx_fund_ecdsa = substrate::tx()
        .balances()
        .transfer(ecdsa_addr, 123_456_789_012_345);
    let hash = api
        .tx()
        .sign_and_submit_default(&tx_fund_ecdsa, &alice_signer)
        .await?;
    println!("Alice sending funds to ECDSA account: {hash}");

    let bob_addr = AccountKeyring::Bob.to_account_id().into();

    let tx_spend_ecdsa = substrate::tx().balances().transfer(bob_addr, 123_456);
    let hash = api
        .tx()
        .sign_and_submit_default(&tx_spend_ecdsa, &ecdsa_signer)
        .await?;
    println!("ECDSA account sending funds to Bob: {hash}");

    Ok(())
}
