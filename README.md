# domain_reg

Check domain avaliablity for registration  

## Build  

This tool requires rust  

```bash
git clone https://github.com/melbadry9/domain_reg.git
cd domain_reg
cargo build --release
```

## Usage

```bash
cat list.txt | dom_reg -t 3 | tee -a status.txt
dom_reg -d test.com -t 3
```
