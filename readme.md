# fbar_prep

A reimplementation of https://github.com/flyinggrizzly/fbar-prep in Rust, with the goals:

- make the tool more portable
- make the tool easier to use, and not dependent on having data inside its own directory structure
- (learn Rust)

## usage

1. put all your statements into a single directory
2. define `accounts.yml` and `mapping.yml` files as necessary
3. run `% fbp fbar_data [--outdir=~/Dropbox/fbar_reports]`
4. file your reports, using the `OUTDIR/fbp_fbar_report_DATE.csv`
