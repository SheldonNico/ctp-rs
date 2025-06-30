use ctp_rs as sys;
use std::ffi;
use std::sync::mpsc::{self, Receiver, Sender, SyncSender};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Resume {
    Restart = sys::THOST_TE_RESUME_TYPE_THOST_TERT_RESTART as _,
    Resume = sys::THOST_TE_RESUME_TYPE_THOST_TERT_RESUME as _,
    Quick = sys::THOST_TE_RESUME_TYPE_THOST_TERT_QUICK as _,
}

#[derive(Debug, Clone, Default)]
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

    qry_freq: i32,
}

pub struct TDApi {
    api: *mut sys::CThostFtdcTraderApi,
    spi: Option<*mut sys::Rust_Td_CThostFtdcTraderSpi>,
    rx: Option<Receiver<String>>,

    pub(crate) config: Config,
}

impl TDApi {
    pub fn get_version() -> String {
        let cs = unsafe { ffi::CStr::from_ptr(sys::CThostFtdcTraderApi::GetApiVersion()) };
        cs.to_string_lossy().into()
    }

    pub fn new(config: &Config) -> Self {
        let cs = std::ffi::CString::new(config.flowpath.as_bytes()).unwrap();
        let api = unsafe { sys::CThostFtdcTraderApi_CreateFtdcTraderApi(cs.as_ptr()) };
        assert!(!api.is_null());
        Self {
            api,
            spi: None,
            config: config.clone(),
            rx: None,
        }
    }

    /// destory `self.spi`, which created by `TDApi`
    fn drop_spi(spi: *mut sys::Rust_Td_CThostFtdcTraderSpi) {
        let mut spi = unsafe { Box::from_raw(spi) };
        unsafe {
            spi.destruct();
        }
    }

    fn register(&mut self, tx: SyncSender<String>) {
        if let Some(spi) = self.spi.take() {
            log::debug!("des old registered spi");
            Self::drop_spi(spi);
        }

        let param = Box::into_raw(Box::new(tx)) as *mut ffi::c_void;

        let spi = unsafe { sys::Rust_Td_CThostFtdcTraderSpi::new(Some(TDApi::callback), param) };
        let spi = Box::into_raw(Box::new(spi));

        unsafe {
            ((*(*self.api).vtable_).CThostFtdcTraderApi_RegisterSpi)(
                self.api,
                spi as *mut sys::CThostFtdcTraderSpi,
            );
        }

        self.spi = Some(spi);
    }

    unsafe extern "C" fn callback(
        msg_type: sys::Rust_Td_MsgType,
        msg_body: sys::Rust_Td_MsgBody,
        params: *mut ::std::os::raw::c_void,
    ) {
        eprintln!("Got callback: {:?}", msg_type);

        match (msg_type, msg_body) {
            (
                sys::Rust_Td_MsgType::OnFrontConnected,
                sys::Rust_Td_MsgBody {
                    // on_front_disconnected: msg,
                    on_front_connected: _,
                },
            ) => {
                println!("Got connected ...");
            }
            (sys::Rust_Td_MsgType::OnRspError, sys::Rust_Td_MsgBody { arg3 }) => {
                let info = arg3.body as *const sys::CThostFtdcRspInfoField;
                println!("Got OnRspError: {:?}", arg3);
                if !info.is_null() {
                    let code = (&*info).ErrorID;
                    let msg = ffi::CStr::from_ptr((&*info).ErrorMsg.as_ptr());
                    println!("\tInfo: {:?} vs{:?}", code, msg);
                }
            }
            _ => {}
        }
    }

    pub fn req_init(&mut self) -> Result<(), String> {
        let (tx, rx) = mpsc::sync_channel(1024);
        self.register(tx);
        self.rx = Some(rx);
        log::debug!("start api...");

        let cs = ffi::CString::new(self.config.front_addr.as_bytes()).unwrap();
        if self.config.front_addr.len() > 0 {
            log::debug!("cs is: {}", self.config.front_addr);
            // let cs = ffi::CString::new(self.config.front_addr.as_bytes()).unwrap();
            unsafe {
                ((*(*self.api).vtable_).CThostFtdcTraderApi_RegisterFront)(
                    self.api,
                    cs.as_ptr() as *mut _,
                );
            }
        }
        log::debug!("subscribing...");

        eprintln!(">>>>>>>");
        unsafe {
            ((*(*self.api).vtable_).CThostFtdcTraderApi_SubscribePrivateTopic)(
                self.api,
                self.config.private_resume as _,
            );
            eprintln!(">>>>>>>");

            ((*(*self.api).vtable_).CThostFtdcTraderApi_SubscribePublicTopic)(
                self.api,
                self.config.public_resume as _,
            );
        }
        eprintln!(">>>>>>>");

        log::debug!("init...");
        unsafe {
            ((*(*self.api).vtable_).CThostFtdcTraderApi_Init)(self.api);
        }

        Ok(())
    }
}

impl Drop for TDApi {
    fn drop(&mut self) {
        log::debug!("drop api");
        unsafe {
            ((*(*self.api).vtable_).CThostFtdcTraderApi_Release)(self.api);
        }
        if let Some(spi) = self.spi {
            log::debug!("drop spi");
            Self::drop_spi(spi);
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

fn main() {
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

        // 要确保跟服务器的版本兼容，否则会报 segment fault
        //
        // 旧的 simnow 地址已经关停了
        front_addr: "tcp://180.169.241.154:41405".into(),
        ..Default::default()
    });
    tdapi.req_init().unwrap();

    log::info!("mk api success");
    // std::thread::sleep(std::time::Duration::from_secs(5));
    if let Some(ref mut rx) = tdapi.rx {
        while let Ok(event) = rx.recv() {
            log::debug!("Got event: {}", event);
        }
    }
}

impl Default for Resume {
    fn default() -> Self {
        Self::Quick
    }
}
