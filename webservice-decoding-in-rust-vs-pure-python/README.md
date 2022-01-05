# Rust 🦀 vs Python 🐍: Webservice with decoding HTTP Body in Rust and then calling Python vs pure Python

This repository contains the code to my blog post [Rust 🦀 vs Python 🐍: JSON decoding in rust and calling python vs pure python](). It includes the code to deploy both Webservice as well as the benchmarking script.

## Code

- [Python](./python)
- [Rust](./rust)

## Deployment

**Python**

```bash
cd python && uvicorn app:app --port 8080
```

single

```bash
curl --request POST \
  --url http://127.0.0.1:8080/age \
  --header 'Content-Type: application/json' \
  --data '{
	"inputs": "I love you. I like you. I am your friend."
}'
```

**Rust**

build dev container
```Bash
docker build -f dockerfile -t actix-python . 
```
run dev container
```Bash
	docker run -v $(pwd):/code -p 8080:8080 -ti actix-python 
```


```bash
cd rust && cargo build --release

chmod +x ./target/release/webservice
./target/release/webservice
```

## Benchmarking

For Benchmarking i used [hey](https://github.com/rakyll/hey)

```bash
hey -n 1000 -m POST -H 'Content-Type: application/json' -d '{	"inputs": "I love you. I like you. I am your friend."}' http://127.0.0.1:8080/age
```

## Results

|              | Python     | Rust Debug  | Rust Release | Difference |
|--------------|------------|-------------|--------------|------------|
| Total        | 18.8745     | 0,0460      | 0,0218       | -96,74%    |
| Slowest      | 1.1446     | 0,0109      | 0,0065       | -84,34%    |
| Fastest      | 0.0237     | 0,0003      | 0,0001       | -99,43%    |
| Average      | 0.9137     | 0,0021      | 0,0009       | -97,23%    |
| Requests/sec | 52.9814 | 21.746,6538 | 45.874,3437  | 2967,35%   |

![requests-per-second](req_sec.png)

### Rust

#### Debug

```bash
Response time histogram:
  0.000 [1]     |
  0.001 [348]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.002 [357]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.003 [164]   |■■■■■■■■■■■■■■■■■■
  0.005 [76]    |■■■■■■■■■
  0.006 [35]    |■■■■
  0.007 [10]    |■
  0.008 [7]     |■
  0.009 [0]     |
  0.010 [0]     |
  0.011 [2]     |
```

#### Release

```bash
Response time histogram:
  0.000 [1]     |
  0.001 [590]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  0.001 [226]   |■■■■■■■■■■■■■■■
  0.002 [61]    |■■■■
  0.003 [70]    |■■■■■
  0.003 [39]    |■■■
  0.004 [4]     |
  0.005 [7]     |
  0.005 [1]     |
  0.006 [0]     |
  0.007 [1]     |
```

### Python

```bash
Response time histogram:
  0.024 [1]     |
  0.136 [5]     |
  0.248 [13]    |■
  0.360 [1]     |
  0.472 [4]     |
  0.584 [6]     |
  0.696 [6]     |
  0.808 [46]    |■■
  0.920 [45]    |■■
  1.033 [867]   |■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■■
  1.145 [6]     |
```