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

    pub fn swe_utc_to_jd(
        iyear: c_int,
        imonth: c_int,
        iday: c_int,
        ihour: c_int,
        imin: c_int,
        dsec: c_double,  /* NOTE: second is a decimal */
        gregflag: c_int, /* Gregorian calendar: 1, Julian calendar: 0 */
        dret: *const c_double, /* return array, two doubles:
                          * dret[0] = Julian day in ET (TT)
                          * dret[1] = Julian day in UT (UT1) */
        serr: *const c_char, /* error string */
    ) -> c_int;

    

    pub fn swe_houses(
        tjd_ut: c_double, /* Julian day number, UT */
        geolat: c_double, /* geographic latitude, in degrees */
        geolon: c_double, /* geographic longitude, in degrees
                           * eastern longitude is positive,
                           * western longitude is negative,
                           * northern latitude is positive,
                           * southern latitude is negative */
        hsys: c_int, /* house method, ascii code of one of the letters documented below */
        cusps: *const c_double, /* array for 13 (or 37 for system G) doubles, explained further below */
        ascmc: *const c_double, /* array for 10 doubles, explained further below */
    ) -> c_int;

}
