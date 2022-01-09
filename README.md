# ysoserial_rs

- ysoserial的rust实现
- 去特征，比原版体积小一半
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

| Gadget              | Support |
|---------------------|---------|
| BeanShell1          | ✅       |
| C3P0                | ❌       |
| Clojure             | ✅       |
| CommonsBeanutils1   | ✅       |
| CommonsCollections1 | ✅       |
| CommonsCollections2 | ✅       |
| CommonsCollections3 | ✅       |
| CommonsCollections4 | ✅       |
| CommonsCollections5 | ✅       |
| CommonsCollections6 | ✅       |
| CommonsCollections7 | ✅       |
| FileUpload1         | ❌       |
| Groovy1             | ✅       |
| Hibernate1          | ✅       |
| Hibernate2          | ✅       |
| JBossInterceptors1  | ✅       |
| JRMPClient          | ❌       |
| JRMPListener        | ❌       |
| JSON1               | ✅       |
| JavassistWeld1      | ✅       |
| Jdk7u21             | ✅       |
| Jython1             | ❌       |
| MozillaRhino1       | ✅       |
| MozillaRhino2       | ✅       |
| Myfaces1            | ✅       |
| Myfaces2            | ❌       |
| ROME                | ✅       |
| Spring1             | ✅       |
| Spring2             | ✅       |
| URLDNS              | ✅       |
| Vaadin1             | ✅       |
| Wicket1             | ❌       |
| Jdk8u20             | ✅       |

## 感谢

- 灵感：[Gososerial](https://github.com/EmYiQing/Gososerial)
- 开发工具：Meld，wxHexEditor,xxd