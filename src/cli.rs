use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "muttmates",
    about = "Utility to retrieve emails from VCards in a mutt compatible fashion"
)]
pub struct Opt {
    /// query
    pub query: Option<String>,
    /// file or folder containing your vcf files
    #[structopt(short = "-c", long = "--contacts", parse(from_os_str))]
    pub contacts: Option<PathBuf>,
}
