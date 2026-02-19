use std::{
    fs,
    path::{Path, PathBuf},
    sync::Arc,
};

use bucket::config::{Args as BucketArgs, InitArgs as BucketInitArgs};
use candid::{decode_one, CandidType, Deserialize, Principal};
use directory::{
    config::{Args as DirectoryArgs, InitArgs as DirectoryInitArgs},
    results::ProvisionBucketResult,
};
use pocket_ic::{PocketIc, WasmResult};

/// Common methods for interacting with a canister using `PocketIc`.
pub trait PicCanisterTrait {
    /// A shared PocketIc instance.
    fn pic(&self) -> Arc<PocketIc>;

    /// The ID of this canister.
    fn canister_id(&self) -> Principal;

    /// Makes an update call to the canister.
    #[allow(dead_code)]
    fn update<T>(
        &self,
        caller: Principal,
        method: &str,
        arg: impl candid::utils::ArgumentEncoder,
    ) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a> + CandidType,
    {
        let arg_bytes =
            candid::encode_args(arg).map_err(|e| format!("Candid encoding failed: {}", e))?;
        self.pic()
            .update_call(self.canister_id(), caller, method, arg_bytes)
            .map_err(|e| {
                format!(
                    "Update call error. RejectionCode: {:?}, Error: {}",
                    e.code, e.description
                )
            })
            .and_then(|bytes| match bytes {
                WasmResult::Reply(b) => decode_one(&b).map_err(|e| format!("Decoding failed: {e}")),
                WasmResult::Reject(reject) => Err(format!("Canister rejected: {reject}")),
            })
    }

    /// Makes a query call to the canister.
    #[allow(dead_code)]
    fn query<T>(
        &self,
        caller: Principal,
        method: &str,
        arg: impl candid::utils::ArgumentEncoder,
    ) -> Result<T, String>
    where
        T: for<'a> Deserialize<'a> + CandidType,
    {
        let arg_bytes =
            candid::encode_args(arg).map_err(|e| format!("Candid encoding failed: {}", e))?;
        self.pic()
            .query_call(self.canister_id(), caller, method, arg_bytes)
            .map_err(|e| {
                format!(
                    "Query call error. RejectionCode: {:?}, Error: {}",
                    e.code, e.description
                )
            })
            .and_then(|bytes| match bytes {
                WasmResult::Reply(b) => decode_one(&b).map_err(|e| format!("Decoding failed: {e}")),
                WasmResult::Reject(reject) => Err(format!("Canister rejected: {reject}")),
            })
    }

    #[allow(dead_code)]
    fn workspace_dir() -> PathBuf {
        let output = std::process::Command::new(env!("CARGO"))
            .arg("locate-project")
            .arg("--workspace")
            .arg("--message-format=plain")
            .output()
            .unwrap()
            .stdout;
        let cargo_path = Path::new(std::str::from_utf8(&output).unwrap().trim());
        cargo_path.parent().unwrap().to_path_buf()
    }

    /// The path to a typical Cargo Wasm build.
    #[allow(dead_code)]
    fn cargo_wasm_path(name: &str) -> String {
        let workspace_dir = Self::workspace_dir();
        workspace_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(name)
            .with_extension("wasm")
            .to_str()
            .unwrap()
            .to_string()
    }
}

/// A typical canister running on PocketIC.
#[derive(Clone)]
pub struct PicCanister {
    pub pic: Arc<PocketIc>,
    pub canister_id: Principal,
}

impl PicCanisterTrait for PicCanister {
    /// The shared PocketIc instance.
    fn pic(&self) -> Arc<PocketIc> {
        self.pic.clone()
    }

    /// The ID of this canister.
    fn canister_id(&self) -> Principal {
        self.canister_id
    }
}

impl PicCanister {
    /// Creates a new canister.
    #[allow(dead_code)]
    pub fn new(pic: Arc<PocketIc>, wasm_path: &str) -> Self {
        PicCanisterBuilder::default()
            .with_wasm(wasm_path)
            .deploy_to(pic)
    }

    /// Helper for proxy calls with cycles
    #[allow(dead_code)]
    pub fn update_with_cycles<T: for<'a> Deserialize<'a> + CandidType>(
        &self,
        proxy: &PicCanister,
        caller: Principal,
        method: &str,
        arg: impl candid::utils::ArgumentEncoder,
        cycles: u128,
    ) -> Result<T, String> {
        let arg_bytes =
            candid::encode_args(arg).map_err(|e| format!("Candid encoding failed: {}", e))?;

        // proxy_call(canister_id, method, args, cycles)
        let proxy_args = (self.canister_id, method.to_string(), arg_bytes, cycles);

        // proxy_call returns Vec<u8>
        let res_bytes: Vec<u8> = proxy.update(caller, "proxy_call", proxy_args)?;

        // We decode it as a sequence containing one value T
        let (res,): (T,) = candid::decode_args(&res_bytes)
            .map_err(|e| format!("Candid decoding failed: {}", e))?;
        Ok(res)
    }
}

#[derive(Debug)]
pub struct PicCanisterBuilder {
    canister_id: Option<Principal>,
    cycles: u128,
    wasm_path: String,
    arg: Vec<u8>,
    controllers: Option<Vec<Principal>>,
}

// Defaults
impl PicCanisterBuilder {
    pub const DEFAULT_CYCLES: u128 = 2_000_000_000_000;

    pub fn default_arg() -> Vec<u8> {
        candid::encode_args(()).unwrap()
    }
}

impl Default for PicCanisterBuilder {
    fn default() -> Self {
        Self {
            canister_id: None,
            cycles: Self::DEFAULT_CYCLES,
            wasm_path: "unspecified.wasm".to_string(),
            arg: Self::default_arg(),
            controllers: None,
        }
    }
}

// Customisation
impl PicCanisterBuilder {
    #[allow(dead_code)]
    pub fn with_arg(mut self, arg: impl candid::utils::ArgumentEncoder) -> Self {
        self.arg = candid::encode_args(arg).unwrap();
        self
    }

    #[allow(dead_code)]
    pub fn with_canister(mut self, canister_id: Principal) -> Self {
        self.canister_id = Some(canister_id);
        self
    }

    #[allow(dead_code)]
    pub fn with_controllers(mut self, controllers: Vec<Principal>) -> Self {
        self.controllers = Some(controllers);
        self
    }

    #[allow(dead_code)]
    pub fn with_cycles(mut self, cycles: u128) -> Self {
        self.cycles = cycles;
        self
    }

    pub fn with_wasm(mut self, wasm_path: &str) -> Self {
        self.wasm_path = wasm_path.to_string();
        self
    }
}

// Get parameters
impl PicCanisterBuilder {
    #[allow(dead_code)]
    fn wasm_bytes(&self) -> Vec<u8> {
        fs::read(self.wasm_path.clone())
            .unwrap_or_else(|_| panic!("Could not find the backend wasm: {}", self.wasm_path))
    }
}

// Builder
impl PicCanisterBuilder {
    #[allow(dead_code)]
    fn get_or_create_canister_id(&mut self, pic: &PocketIc) -> Principal {
        if let Some(canister_id) = self.canister_id {
            canister_id
        } else {
            let canister_id = pic.create_canister();
            self.canister_id = Some(canister_id);
            canister_id
        }
    }

    fn add_cycles(&mut self, pic: &PocketIc) {
        if self.cycles > 0 {
            let canister_id = self.get_or_create_canister_id(pic);
            pic.add_cycles(canister_id, self.cycles);
        }
    }

    fn install(&mut self, pic: &PocketIc) {
        let wasm_bytes = self.wasm_bytes();
        let canister_id = self.get_or_create_canister_id(pic);
        let arg = self.arg.clone();
        pic.install_canister(canister_id, wasm_bytes, arg, None);
    }

    fn set_controllers(&mut self, pic: &PocketIc) {
        if let Some(controllers) = self.controllers.clone() {
            let canister_id = self.get_or_create_canister_id(pic);
            pic.set_controllers(canister_id, None, controllers)
                .expect("Test setup error: Failed to set controllers");
        }
    }

    pub fn deploy_to(&mut self, pic: Arc<PocketIc>) -> PicCanister {
        let canister_id = self.get_or_create_canister_id(&pic);
        self.add_cycles(&pic);
        self.install(&pic);
        self.set_controllers(&pic);
        PicCanister {
            pic: pic.clone(),
            canister_id,
        }
    }
}

#[allow(dead_code)]
pub struct TestSetup {
    pub pic: Arc<PocketIc>,
    pub directory: PicCanister,
    pub bucket: PicCanister,
    pub proxy: PicCanister,
}

impl Default for TestSetup {
    fn default() -> Self {
        let pic = Arc::new(PocketIc::new());

        // We need these for directory init
        let icp_ledger = Principal::anonymous();
        let ckusdc_ledger = Principal::anonymous();

        // 1. Deploy Proxy (No arguments)
        let proxy = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("test_proxy"))
            .with_cycles(1_000_000_000_000_000)
            .deploy_to(pic.clone());

        // 2. Deploy Bucket
        let bucket_init_args = (BucketArgs::Init(BucketInitArgs {
            icp_ledger,
            ckusdc_ledger,
        }),);
        let bucket = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("bucket"))
            .with_arg(bucket_init_args)
            .deploy_to(pic.clone());

        // 3. Deploy Directory
        let directory_init_args = (DirectoryArgs::Init(DirectoryInitArgs {
            icp_ledger,
            ckusdc_ledger,
            rate_per_gb_per_month: 100_000_000, // 0.1 tokens per GB
        }),);
        let directory = PicCanisterBuilder::default()
            .with_wasm(&PicCanister::cargo_wasm_path("directory"))
            .with_arg(directory_init_args)
            .deploy_to(pic.clone());

        // 4. Provision bucket in directory
        let _: ProvisionBucketResult = directory
            .update(
                Principal::anonymous(),
                "provision_bucket",
                (bucket.canister_id,),
            )
            .unwrap();

        Self {
            pic,
            directory,
            bucket,
            proxy,
        }
    }
}
