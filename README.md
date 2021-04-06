# domain_reg
Check Domain Avaliablity 

## Build 

```bash
git clone https://github.com/melbadry9/domain_reg.git
cd domain_reg
cargo build --release
```

## Usage 

```bash
cat list.txt | dom_reg | tee -a status.txt
```
