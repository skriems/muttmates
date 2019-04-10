use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "vcard", about = "Utility to parse VCards")]
pub struct Opt {
    /// query
    pub query: Option<String>,
    /// file or folder containing your vcf files
    #[structopt(short = "-c", long = "--contacts", parse(from_os_str))]
    pub contacts: Option<PathBuf>,
}
