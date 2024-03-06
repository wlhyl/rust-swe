# rust-swe
瑞士星历表的rust绑定。只实现了以下函数。
* swe_version
* swe_set_ephe_path
* fn swe_close
* swe_date_conversion
* swe_calc_ut
* swe_julday
* swe_revjul
* swe_utc_time_zone
* swe_utc_to_jd
* swe_degnorm
* swe_houses
* swe_cotrans
* swe_fixstar2_ut

# 安装
* 安装swe
```
mkdir /tmp/swe
cd /tmp/swe
wget https://github.com/aloistr/swisseph/archive/refs/tags/v2.10.03.tar.gz -O swe.tar.gz
tar xvzf swe.tar.gz
cd swisseph-2.10.03
make libswe.a
```
* 在rust项目中添加以下依赖
```
[dependencies]
...
rust-swe = { git = "https://github.com/wlhyl/rust-swe.git" }
```
