# ctp-rs
A simple demo for use bindgen with ctp sdk.

```
$ # build with CTP
$ CTP_SDK_PATH=/home/xnie/projects/ctp-rs/shared/sdk/v6.7.9_P1_20250319_traderapi/v6.7.9_P1_20250319_traderapi/v6.7.9_P1_20250319_api_traderapi_linux64/v6.7.9_P1_20250319_api/v6.7.9_P1_20250319_api_traderapi_se_linux64 cargo b --example tdapi -vvv && LD_LIBRARY_PATH=/home/xnie/projects/ctp-rs/shared/sdk/v6.7.9_P1_20250319_traderapi/v6.7.9_P1_20250319_traderapi/v6.7.9_P1_20250319_api_traderapi_linux64/v6.7.9_P1_20250319_api/v6.7.9_P1_20250319_api_traderapi_se_linux64 ./target/debug/examples/tdapi

$ # build with CTP mini 
$ CTP_SDK_PATH=/home/xnie/projects/ctp-rs/shared/sdk/CTP_Mini_V1.7.0_20240927/CTP\ Mini_V1.7.0_20240927/CTP\ Mini_V1.7.0_20240927/CTP\ Mini_V1.7.0_linux64_api_20240923/CTP\ Mini_V1.7.0_linux64_api_20240923/ cargo b --example tdapi -vvv
```
