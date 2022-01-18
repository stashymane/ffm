use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long, long_help = "Path to a Firefox profile.", help = "Profile path")]
    profile: String,

    #[clap(short, long, long_help = "Installs packages from the specified locations.", help = "Install packages")]
    install: Vec<String>,

    #[clap(short, long, long_help = "Removes packages with the specified IDs.", help = "Remove packages")]
    uninstall: Vec<String>,

    #[clap(short, long, long_help = "Enables packages with the specified IDs.", help = "Enable packages")]
    enable: Vec<String>,

    #[clap(short, long, long_help = "Disables packages with the specified IDs.", help = "Disable packages")]
    disable: Vec<String>,
}

fn main() {
    let args = Args::parse();
}