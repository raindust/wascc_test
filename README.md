## Compile providers
### http-server-provider
Enter into `http-server-provider` sub directory and run:
```shell
cargo build
```
Back to the top directory and run following command to copy:
```shell
cp http-server-provider/target/debug/libwascc_httpsrv.dylib .
```
### keyvalue
Enter into `keyvalue` sub directory and run:
```shell
cargo build
```
Back to the top directory and run following command to copy:
```shell
cp keyvalue/target/debug/libkeyvalue.dylib .
```
## Compile Actor
Enter into `hellohttp` sub directory and run:
```shell
cargo build
```
then run the following script to sign the actor wasm:
```shell
./generate_signed.sh
```
## Test with httprunner
Enter into `hellorunner` sub directory and run:
```shell
cargo run
```
then go to a browser and access http://localhost:8081/ URL, notice prints about the running `hellorunner` app.