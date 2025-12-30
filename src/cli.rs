use clap::{Args, Parser, Subcommand};


#[derive(Debug, Parser, Clone)]
pub struct Cli{
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Cmd{
    Config(ConfigCmd),
    Generate(GenerateCmd)
}

#[derive(Debug, Clone, Args)]
pub struct ConfigCmd{
    /// Give the surah number from which to strat asking from
    #[arg(long = "from-surah")]
    pub from_surah: Option<usize>,
    /// Give the ayah number from which to start asking from
    #[arg(long = "from-ayah")]
    pub from_ayah: Option<usize>,
    /// Give the surah number up to which to ask
    #[arg(long = "upto-surah")]
    pub upto_surah: Option<usize>,
    /// Give the ayah number up to which to ask
    #[arg(long = "upto-ayah")]
    pub upto_ayah: Option<usize>,
}

#[derive(Debug, Clone, Args)]
pub struct GenerateCmd{
    /// Number of questions to generate
    #[arg(short, long)]
    pub number: Option<usize>,
}