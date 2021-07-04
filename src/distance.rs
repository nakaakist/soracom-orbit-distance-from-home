use std::f64::consts::PI;

pub struct Location {
    pub lat_deg: f64,
    pub lon_deg: f64,
}

// ２点の緯度経度(deg.)から、距離(km)を計算する。
// 計算の公式はhttps://keisan.casio.jp/exec/system/1257670779　を参照した。
pub fn calc(location1: Location, location2: Location) -> f64 {
    let lat1_rad = to_rad(location1.lat_deg);
    let lon1_rad = to_rad(location1.lon_deg);
    let lat2_rad = to_rad(location2.lat_deg);
    let lon2_rad = to_rad(location2.lon_deg);

    let earth_radius = 6138.137;

    return earth_radius
        * (lat1_rad.sin() * lat2_rad.sin()
            + lat1_rad.cos() * lat2_rad.cos() * (lon1_rad - lon2_rad).cos())
        .acos();
}

fn to_rad(deg: f64) -> f64 {
    return deg / 180.0 * PI;
}
