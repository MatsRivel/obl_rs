mod make_1bill_rows;
mod obr;
use crate::obr::obr_challenge;
use crate::make_1bill_rows::row_builder;
fn main() {
    // row_builder().unwrap();
    obr_challenge("src/obr.csv").unwrap()
}
