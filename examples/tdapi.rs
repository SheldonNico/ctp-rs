use ctp_rs::sys::*;

use std::time::{Duration, Instant};
use std::io::{Write, Read};
use std::os::raw::*;
use std::ffi::{CStr, CString};
use std::collections::{HashMap, HashSet};
use std::path::Path;
use std::sync::{Arc, Mutex, Condvar};

use log::*;
use crossbeam::{channel::{self, Sender, Receiver}, select};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum Resume {
    Restart = THOST_TE_RESUME_TYPE_THOST_TERT_RESTART as _,
    Resume = THOST_TE_RESUME_TYPE_THOST_TERT_RESUME as _,
    Quick = THOST_TE_RESUME_TYPE_THOST_TERT_QUICK as _,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    flowpath: String,
    front_addr: String,
    nm_addr: String,
    user_info: String,
    product_info: String,
    auth_code: String,
    app_id: String,
    public_resume: Resume,
    private_resume: Resume,

    broker_id: String,
    user_id: String,
    password: String,

    #[serde(default)]
    qry_freq: i32,
}

pub struct TDApi {
    api: Rust_CThostFtdcTraderApi,
    spi: Option<*mut Rust_CThostFtdcTraderSpi>,
    rx:  Option<Receiver<String>>,

    pub(crate) config: Config,
}

struct Spi {
    tx: Sender<String>
}

impl Rust_CThostFtdcTraderSpi_Trait for Spi {
    fn on_front_connected(&mut self) {
        debug!("connected.");
        self.tx.send("connected".into()).unwrap();
    }
}

impl TDApi {
    pub fn get_version() -> String {
        let cs = unsafe { CStr::from_ptr(CThostFtdcTraderApi::GetApiVersion()) };
        cs.to_string_lossy().into()
    }

    pub fn new(config: &Config) -> Self {
        let cs = std::ffi::CString::new(config.flowpath.as_bytes()).unwrap();
        let api = unsafe {
            Rust_CThostFtdcTraderApi::new(CThostFtdcTraderApi_CreateFtdcTraderApi(cs.as_ptr()))
        };
        Self { api, spi: None, config: config.clone(), rx: None }
    }

    /// destory `self.spi`, which created by `TDApi`
    fn drop_spi(spi: *mut Rust_CThostFtdcTraderSpi) {
        let mut spi = unsafe { Box::from_raw(spi) };
        unsafe { spi.destruct(); }
    }

    fn register<S: Rust_CThostFtdcTraderSpi_Trait>(&mut self, spi: S) {
        if let Some(spi) = self.spi.take() {
            debug!("des old registered spi");
            Self::drop_spi(spi);
        }

        let spi: Box<Box<dyn Rust_CThostFtdcTraderSpi_Trait>> = Box::new(Box::new(spi));
        let ptr = Box::into_raw(spi) as *mut _ as *mut c_void;

        // Here is the trick part:
        //
        // 1. `RegisterSpi` recv a `CThostFtdcTraderSpi*`, which can be cast from
        // Rust_CThostFtdcTraderSpi, because `Rust_CThostFtdcTraderSpi` inherit from `CThostFtdcTraderSpi`
        // in wrapper.hpp.
        //
        // 2. What `RegisterSpi` recved must be valid afterwards. We can use pin or boxed pointer to achieve
        // that. What we can't do is code below, which use `Rust_CThostFtdcTraderSpi` as spi instead of a
        // pointer `*mut Rust_CThostFtdcTraderSpi`:
        //
        // ```rust
        // let mut spi_stub = unsafe { Rust_CThostFtdcTraderSpi::new(ptr) } ;
        // let spi: *mut Rust_CThostFtdcTraderSpi = &mut spi_stub;
        //
        // unsafe { self.api.RegisterSpi(spi as _); }
        //
        // self.spi = Some(spi_stub);
        // ```
        //
        // but we can do this:
        //
        // ```rust
        // let mut spi_stub = unsafe { Rust_CThostFtdcTraderSpi::new(ptr) } ;
        // self.spi = Some(spi_stub);
        // let spi: *mut Rust_CThostFtdcTraderSpi = self.spi.as_mut().unwrap() as *mut _;
        //
        // unsafe { self.api.RegisterSpi(spi as _); }
        // ```
        //
        // Because original code move spi_stub into self.spi after RegisterSpi, which cause original
        // address invalid.
        let spi_stub = unsafe { Rust_CThostFtdcTraderSpi::new(ptr) } ;
        let spi: *mut Rust_CThostFtdcTraderSpi = Box::into_raw(Box::new(spi_stub));
        unsafe { self.api.RegisterSpi(spi as _); }

        self.spi = Some(spi);
    }

    pub fn req_init(&mut self) -> Result<(), String> {
        let (tx, rx) = channel::bounded(1024);
        self.register(Spi { tx });
        self.rx = Some(rx);
        debug!("start api...");

        if self.config.front_addr.len() > 0 {
            debug!("cs is: {}", self.config.front_addr);
            let cs = CString::new(self.config.front_addr.as_bytes()).unwrap();
            unsafe { self.api.RegisterFront(cs.as_ptr() as *mut _); }
        }

        unsafe {
            self.api.SubscribePrivateTopic(self.config.private_resume as _);
            self.api.SubscribePublicTopic(self.config.public_resume as _);
        }

        unsafe { self.api.Init(); }

        Ok(())
    }


}

impl Default for Resume {
    fn default() -> Self {
        Self::Quick
    }
}

impl Drop for TDApi {
    fn drop(&mut self) {
        debug!("drop api");
        unsafe { self.api.destruct(); }
        if let Some(spi) = self.spi {
            debug!("drop spi");
            Self::drop_spi(spi);
        }
    }
}

pub fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    eprintln!("api version: {}", TDApi::get_version());

    let mut tdapi = TDApi::new(&Config {
        flowpath: "".into(),
        nm_addr: "".into(),
        user_info: "".into(),
        product_info: "".into(),
        public_resume: Resume::Quick,
        private_resume: Resume::Quick,

        // simnow - full
        front_addr: "tcp://180.168.146.187:10101".into(),
        broker_id: "9999".into(),
        auth_code: "".into(),
        app_id: "".into(),

        ..Default::default()
    });
    tdapi.req_init().unwrap();

    eprintln!("mk api success");
    if let Some(ref mut rx) = tdapi.rx {
        while let Ok(event) = rx.recv() {
            debug!("Got event: {}", event);
        }
    }

}
