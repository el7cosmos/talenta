use structopt::StructOpt;

#[derive(Default, Debug, StructOpt)]
#[structopt(about = "Request checkin only attendance")]
pub(super) struct Checkin {}
