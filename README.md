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
* swe_degnorm

# 安装
* 安装swe
```
mkdir /tmp/swe
cd /tmp/swe
wget https://www.astro.com/ftp/swisseph/swe_unix_src_2.10.03.tar.gz
tar xvzf swe_unix_src_2.10.03.tar.gz
cd swe
make libswe.so
cp libswe.so /usr/local/lib/
```
* 在rust项目中添加以下依赖
```
[dependencies]
...
rust-swe = { git = "https://github.com/wlhyl/rust-swe.git" }
```
