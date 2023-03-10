use std::os::raw::{c_char, c_double, c_int};
#[link(name = "swe")]
extern "C" {
    pub(crate) fn swe_version(s_version: *const c_char) -> *const c_char;
    pub(crate) fn swe_set_ephe_path(path: *const c_char);
    pub(crate) fn swe_close();
    pub(crate) fn swe_date_conversion(
        y: c_int,
        m: c_int,
        d: c_int,       /* year, month, day */
        hour: c_double, /* hours (decimal, with fraction) */
        c: c_char,      /* calendar ‘g’[regorian]|’j’[ulian] */
        tjd: *const c_double,
    ) -> c_int;

    pub(crate) fn swe_calc_ut(
        tjd_ut: c_double,
        ipl: c_int,
        iflag: c_int,
        xx: *const c_double,
        serr: *const c_char,
    ) -> c_int;

    pub fn swe_julday(
        year: c_int,
        month: c_int,
        day: c_int,
        hour: c_double,
        gregflag: c_int,
    ) -> c_double;

    pub fn swe_revjul(
        tjd: c_double, /* Julian day number */

        gregflag: c_int, /* Gregorian calendar: 1, Julian calendar: 0 */

        year: *const c_int, /* target addresses for year, etc. */

        month: *const c_int,
        day: *const c_int,
        hour: *const c_double,
    );

    pub fn swe_utc_time_zone(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,

        ihour: c_int,
        imin: c_int,
        dsec: c_double,

        d_timezone: c_double,

        iyear_out: *const c_int,
        imonth_out: *const c_int,
        iday_out: *const c_int,

        ihour_out: *const c_int,
        imin_out: *const c_int,
        dsec_out: *const c_double,
    );

    pub fn swe_degnorm(x: c_double) -> c_double;

}
