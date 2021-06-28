# lsm_engine
LSMツリーインデックスベースのKVS  
memcachedプロトコル準拠
## 対応コマンド
- get
- set
- delete
- stats

# usage 
## 起動
```shell
RUST_LOG=DEBUG cargo run --bin server
```

## クライアント
```shell
echo 'set hoge 0 0 11\nhello world' | nc localhost 33333
echo 'get hoge' | nc localhost 33333
echo 'stats' | nc localhost 33333
echo 'delete hoge' | nc localhost 33333
echo 'get hoge' | nc localhost 33333
```
or
```shell
telnet localhost 33333
```
