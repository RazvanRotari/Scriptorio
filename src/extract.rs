use base64::prelude::*;
use libflate::zlib::Decoder;
use serde_json::{to_string_pretty, Value};
use std::io::Read;
mod structs;
use structs::BlueprintWrapper;

fn main() -> anyhow::Result<()> {
    let data= "0eNqNUVtqxDAMvIu+vYWkj01Mb7IUk4d2K0hsI9uhIfjulb2lFAqlf9ZIM5qRDxiXhJ7JRtAH0ORsAH05INDNDkvB4u4RNFDEFRTYYS3VjBPNyKfJrSPZITqGrIDsjB+gm/ymAG2kSHhXq8VubFpHZBn4S0eBd0GozpbtIndq+odnBbu8HmWJWIzsFjPi+7CREGTqS8ZIb67UUNArcYjmV5KNOCZBvk3cJ06McwkRsKj8n3ZjRAvV2OoHrhk0vEIB/C6Wko3mym41ZH2SM0dOmHO5Ub2p/vEFCjbkUKO3XfN07ttz13UvfdPm/Amc5ZML";
    let data = data.split_at(1).1;
    let encoded_data = BASE64_STANDARD.decode(data)?;

    let mut decoder = Decoder::new(&encoded_data[..])?;
    let mut decoded_data = Vec::new();
    decoder.read_to_end(&mut decoded_data)?;
    let json: BlueprintWrapper = serde_json::from_slice(&decoded_data)?;
    println!("{}", to_string_pretty(&json)?);
    Ok(())
}
