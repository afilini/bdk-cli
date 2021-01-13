use std::ops::Deref;

use structopt::StructOpt;

use bip39::{Language, Mnemonic, MnemonicType};

use bdk::bitcoin::util::bip32::DerivationPath;
use bdk::bitcoin::Network;
use bdk::keys::{
    DescriptorKey, DescriptorSecretKey, GeneratableKey, GeneratedKey, ToDescriptorKey,
};
use bdk::miniscript::Segwitv0;

#[derive(Debug, StructOpt, Clone, PartialEq)]
#[structopt(name = "BDK CLI")]
enum AppOpt {
    Generate,
    Convert {
        #[structopt(name = "MNEMONIC", short = "m", long = "mnemonic")]
        mnemonic: String,
        #[structopt(name = "PASSPHRASE", short = "p", long = "passphrase")]
        passphrase: Option<String>,
        #[structopt(
            name = "NETWORK",
            short = "n",
            long = "network",
            default_value = "testnet"
        )]
        network: Network,
    },
}

fn main() {
    let opt = AppOpt::from_args();

    match opt {
        AppOpt::Generate => {
            let key: GeneratedKey<_, Segwitv0> =
                Mnemonic::generate((MnemonicType::Words24, Language::English)).unwrap();
            // dbg!(&key);
            println!("{}", key.deref());
        }
        AppOpt::Convert {
            mnemonic,
            network,
            passphrase,
        } => {
            let mnemonic = Mnemonic::from_phrase(&mnemonic, Language::English).unwrap();
            let desc_key: DescriptorKey<Segwitv0> =
                ((mnemonic, passphrase), DerivationPath::from(vec![]))
                    .to_descriptor_key()
                    .unwrap();

            if let DescriptorKey::Secret(DescriptorSecretKey::XPrv(xkey), _, _) = desc_key {
                let mut xkey = xkey.xkey.clone();
                xkey.network = network;

                dbg!(&xkey);
                println!("{}", xkey);
            }
        }
    }
}
