mod raw {
    use std::os::raw::c_double;
    #[link(name = "swe")]
    extern "C" {
        pub(super) fn swe_cotrans(
            xpo: *const c_double, /* 3 doubles: long., lat., dist. to be converted; distance remains unchanged, can be set to 1.00 */
            xpn: *const c_double, /* 3 doubles: long., lat., dist. Result of the conversion */
            eps: c_double,        /* obliquity of ecliptic, in degrees. */
        );

        pub fn swe_degnorm(x: c_double) -> c_double;

    }
}

/**
 *  黄赤坐标转换
 *  equator -> ecliptic    : eps must be positive
 *  * ecliptic -> equator     : eps must be negative
 *  * eps, longitude and latitude are in positive degrees!
 *  @param long
 *  经度
 *  @param lat
 *  纬度
 *  @param dist
 *  distance remains unchanged, can be set to 1.00
 * 距离保持不变，可设置为1.00
 *  @param eps
 *  黄赤倾角，
 *  赤道->黄道，用正值
 *  黄道->赤首，用负值
 *  @return DoubleArray
 * 返回转换后的long,lat,dist
 */
pub fn swe_cotrans(long: f64, lat: f64, dist: f64, eps: f64) -> [f64; 3] {
    let xpo = [long, lat, dist];
    let xpn = [0.0; 3];
    unsafe {
        raw::swe_cotrans(xpo.as_ptr(), xpn.as_ptr(), eps);
    }
    xpn
}

pub fn swe_degnorm(x: f64) -> f64 {
    unsafe { raw::swe_degnorm(x) }
}