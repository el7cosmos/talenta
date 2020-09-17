use structopt::StructOpt;

#[derive(Default, Debug, StructOpt)]
#[structopt(about = "Request checkout only attendance")]
pub(super) struct Checkout {}
