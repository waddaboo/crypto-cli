use clap::{Args, Parser, Subcommand};
use crypto_cli::rsa::{
    domain::RSAType,
    error::RSAError,
    key::{RSAPrivateKey, RSAPublicKey},
};
use num_bigint::BigUint;

#[derive(Parser)]
#[command(author, version)]
#[command(about = "A CLI tool that can generate RSA key pair, encrypt and decrypt message.")]

struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Generate(Generate),
    Encrypt(Encrypt),
    Decrypt(Decrypt),
    EncryptEncode(EncryptEncode),
    DecryptDecode(DecryptDecode),
}

#[derive(Args, Debug)]
struct Generate {
    string: Option<bool>,
}

#[derive(Args)]
#[group(required = true)]
struct Encrypt {
    #[arg(short = 't', long = "text", required = true)]
    text: String,
    #[arg(short = 'p', long = "public_exponent", required = true)]
    public_exponent: BigUint,
    #[arg(short = 'm', long = "modulus", required = true)]
    modulus: BigUint,
}

#[derive(Args)]
#[group(required = true)]
struct Decrypt {
    #[arg(short = 't', long = "text")]
    text: String,
    #[arg(short = 'p', long = "private_exponent", required = true)]
    private_exponent: BigUint,
    #[arg(short = 'e', long = "public_exponent", required = true)]
    public_exponent: BigUint,
    #[arg(short = 'm', long = "modulus", required = true)]
    modulus: BigUint,
}

#[derive(Args)]
#[group(required = true)]
struct EncryptEncode {
    #[arg(short = 't', long = "text", required = true)]
    text: String,
    #[arg(short = 'p', long = "public_exponent", required = true)]
    public_exponent: BigUint,
    #[arg(short = 'm', long = "modulus", required = true)]
    modulus: BigUint,
}

#[derive(Args)]
#[group(required = true)]
struct DecryptDecode {
    #[arg(short = 't', long = "text")]
    text: String,
    #[arg(short = 'p', long = "private_exponent", required = true)]
    private_exponent: BigUint,
    #[arg(short = 'e', long = "public_exponent", required = true)]
    public_exponent: BigUint,
    #[arg(short = 'm', long = "modulus", required = true)]
    modulus: BigUint,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Generate(_)) => {
            let private_key = RSAPrivateKey::new(RSAType::RSA2048).expect("Error generating RSA.");
            let public_key = private_key.to_public_key();

            println!("Private Key: {:#?}\n", private_key);
            println!("Public Key: {:#?}\n", public_key);
        }

        Some(Commands::Encrypt(arg)) => {
            let text = &arg.text;
            let modulus = arg.modulus.clone();
            let public_exponent = arg.public_exponent.clone();

            let public_key = RSAPublicKey {
                modulus,
                public_exponent,
            };

            let encoding_func: Option<fn(Vec<u8>) -> String> = None;

            let ciphertext = public_key
                .encrypt(text, encoding_func)
                .expect("Failed to encrypt text.");

            println!("Encrypted text:\n{}\n", ciphertext);
        }

        Some(Commands::Decrypt(arg)) => {
            let ciphertext = &arg.text;
            let modulus = arg.modulus.clone();
            let public_exponent = arg.public_exponent.clone();
            let private_exponent = arg.private_exponent.clone();

            let private_key = RSAPrivateKey {
                public_key: RSAPublicKey {
                    modulus,
                    public_exponent,
                },
                private_exponent,
            };

            let decoding_func: Option<fn(String) -> Result<Vec<u8>, RSAError>> = None;

            let decrypted_message = private_key
                .decrypt(ciphertext.to_string(), decoding_func)
                .expect("Cannot decrypt text.");

            println!("Decrypted text:\n{}\n", decrypted_message);
        }

        Some(Commands::EncryptEncode(arg)) => {
            let text = &arg.text;
            let modulus = arg.modulus.clone();
            let public_exponent = arg.public_exponent.clone();

            let public_key = RSAPublicKey {
                modulus,
                public_exponent,
            };

            let encoding_func: Option<fn(Vec<u8>) -> String> = Some(hex::encode);

            let ciphertext = public_key
                .encrypt(text, encoding_func)
                .expect("Failed to encrypt text.");

            println!("Encrypted text:\n{}\n", ciphertext);
        }

        Some(Commands::DecryptDecode(arg)) => {
            let ciphertext = &arg.text;
            let modulus = arg.modulus.clone();
            let public_exponent = arg.public_exponent.clone();
            let private_exponent = arg.private_exponent.clone();

            let private_key = RSAPrivateKey {
                public_key: RSAPublicKey {
                    modulus,
                    public_exponent,
                },
                private_exponent,
            };

            let decoding_func: Option<fn(String) -> Result<Vec<u8>, hex::FromHexError>> =
                Some(hex::decode);

            let decrypted_message = private_key
                .decrypt(ciphertext.to_string(), decoding_func)
                .expect("Cannot decrypt text.");

            println!("Decrypted text:\n{}\n", decrypted_message);
        }

        None => {}
    }
}
