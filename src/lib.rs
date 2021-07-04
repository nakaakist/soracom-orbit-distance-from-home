mod distance;
mod home_location;

use soracom_orbit_sdk as orbit;

#[macro_use]
extern crate serde_derive;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[repr(i32)]
#[derive(Debug)]
pub enum ErrorCode {
    Ok = 0,
    ExecError = -1,
}

// デバイスから送られてくるデータ形式
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    lat: f64,
    lon: f64,
    bat: i32,
    rs: i32,
    #[serde(alias = "type")]
    send_type: i32,
}

// SORACOMプラットフォーム側に送るデータ形式
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Output {
    distance_from_home: f64,
    lat: f64,
    lon: f64,
    bat: i32,
    rs: i32,
    #[serde(alias = "type")]
    send_type: i32,
}

// データ変換処理
#[no_mangle]
pub fn uplink() -> ErrorCode {
    let buf = orbit::get_input_buffer();
    let output = process_uplink(buf);
    if let Err(e) = output {
        orbit::log(format!("{}", e).as_str());
        return ErrorCode::ExecError;
    }
    let output = output.unwrap();

    orbit::set_output_json(output.as_str());

    ErrorCode::Ok
}

fn process_uplink(buf: Vec<u8>) -> Result<String, Error> {
    let input: Input = serde_json::from_slice(buf.as_ref())?;

    let output = Output {
        distance_from_home: distance::calc(
            distance::Location {
                lat_deg: input.lat,
                lon_deg: input.lon,
            },
            home_location::LOCATION,
        ),
        lat: input.lat,
        lon: input.lon,
        bat: input.bat,
        rs: input.rs,
        send_type: input.send_type,
    };

    let output_json = serde_json::to_string(&output)?;

    Ok(output_json)
}
