# ctp-rs
CTP sdk version: `6.3.15_20190220 15:47:00`

`*.hpp` and `*.cpp` are generated by `autobind.py`, then processed by [rut_bindgen](https://rust-lang.github.io/rust-bindgen/requirements.html). Look at [xtp-rs](https://github.com/dovahcrow/xtp-rs) for detail.

__Use at your own risk__. The crate is unsafe, look at [examples/tdapi.rs](./examples/tdapi.rs) and [examples/mdapi.rs](./examples/mdapi.rs) for how to use it.

## where to put my .so files
so files is not included, please put it like this:

```
$ tree shared

shared
├── data_collect
│   ├── unix.x86_64
│   │   ├── libLinuxDataCollect.so -> LinuxDataCollect.so
│   │   └── LinuxDataCollect.so
│   ├── windows.x86
│   │   ├── WinDataCollect.dll
│   │   └── WinDataCollect.lib
│   └── windows.x86_x64
│       ├── WinDataCollect.dll
│       └── WinDataCollect.lib
├── include
│   ├── DataCollect.h
│   ├── ThostFtdcMdApi.h
│   ├── ThostFtdcTraderApi.h
│   ├── ThostFtdcUserApiDataType.h
│   └── ThostFtdcUserApiStruct.h
├── md
│   ├── unix.x86_64
│   │   ├── libthostmduserapi_se.so -> thostmduserapi_se.so
│   │   └── thostmduserapi_se.so
│   ├── windows.x86
│   │   ├── thostmduserapi_se.dll
│   │   └── thostmduserapi_se.lib
│   └── windows.x86_x64
│       ├── thostmduserapi_se.dll
│       └── thostmduserapi_se.lib
└── td
    ├── unix.x86_64
    │   ├── libthosttraderapi_se.so -> thosttraderapi_se.so
    │   └── thosttraderapi_se.so
    ├── windows.x86
    │   ├── thosttraderapi_se.dll
    │   └── thosttraderapi_se.lib
    └── windows.x86_x64
        ├── thosttraderapi_se.dll
        └── thosttraderapi_se.lib
```
