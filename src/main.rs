#![allow(non_snake_case)]

use std::{
    ffi::{CString, OsStr},
    os::windows::prelude::OsStrExt,
    ptr::null_mut,
};
use winapi::{
    ctypes::c_void,
    shared::{
        guiddef::{REFCLSID, REFIID},
        minwindef::{BOOL, DWORD, FALSE, FARPROC, HMODULE, LPVOID, UINT},
    },
    um::{
        libloaderapi::{GetProcAddress, LoadLibraryW},
        objidlbase::IEnumUnknown,
        unknwnbase::{IUnknown, IUnknownVtbl},
        winnt::{HANDLE, HRESULT, LONG, LPCSTR, LPCWSTR, LPWSTR},
    },
    DEFINE_GUID, RIDL,
};

// From: https://github.com/jmquigs/ModelMod/tree/006e8b723ba265e2c6d77fe13db28b3b3b10024e/Native/dnclr
DEFINE_GUID! {CLSID_CLR_META_HOST, 0x9280188d, 0xe8e, 0x4867, 0xb3, 0xc, 0x7f, 0xa8, 0x38, 0x84, 0xe8, 0xde}
DEFINE_GUID! {IID_ICLR_META_HOST, 0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16}
DEFINE_GUID! {IID_ICLR_RUNTIME_INFO, 0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91}
DEFINE_GUID! {CLSID_CLR_RUNTIME_HOST, 0x90F1A06E, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}
DEFINE_GUID! {IID_ICLR_RUNTIME_HOST, 0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02}

RIDL!(#[uuid(0xD332DB9E, 0xB9B3, 0x4125, 0x82, 0x07, 0xA1, 0x48, 0x84, 0xF5, 0x32, 0x16)]
interface ICLRMetaHost(ICLRMetaHostVtbl): IUnknown(IUnknownVtbl) {
    fn GetRuntime(pwzVersion:LPCWSTR, riid:REFIID, ppRuntime:*mut *mut ICLRRuntimeInfo,) -> HRESULT,
    fn GetVersionFromFile(pwzFilePath: LPCWSTR, pwzBuffer: LPWSTR, pcchBuffer: *mut DWORD,)
        -> HRESULT,
    fn EnumerateInstalledRuntimes(ppEnumerator: *mut *mut IEnumUnknown,) -> HRESULT,
    fn EnumerateLoadedRuntimes(hndProcess:HANDLE, ppEnumerator: *mut *mut IEnumUnknown,)
        -> HRESULT,
    fn RequestRuntimeLoadedNotification(pCallbackFunction:*mut c_void
        /*RuntimeLoadedCallbackFnPtr*/,) -> HRESULT,
    fn QueryLegacyV2RuntimeBinding( riid:REFIID, ppUnk: *mut *mut c_void,) -> HRESULT,
    fn ExitProcess(iExitCode:u32,) -> HRESULT,
});

RIDL!(#[uuid(0xBD39D1D2, 0xBA2F, 0x486a, 0x89, 0xB0, 0xB4, 0xB0, 0xCB, 0x46, 0x68, 0x91)]
interface ICLRRuntimeInfo(ICLRRuntimeInfoVtbl): IUnknown(IUnknownVtbl) {
    fn GetVersionString(pwzBuffer: LPWSTR, pcchBuffer: *mut DWORD,) -> HRESULT,
    fn GetRuntimeDirectory( pwzBuffer: LPWSTR, pcchBuffer: *mut DWORD,) -> HRESULT,
    fn IsLoaded(hndProcess:HANDLE, pbLoaded: *mut BOOL,) -> HRESULT,
    fn LoadErrorString(iResourceID:UINT, pwzBuffer:LPWSTR, pcchBuffer: *mut DWORD, iLocaleID: LONG,)
        -> HRESULT,
    fn LoadLibrary(pwzDllName:LPCWSTR, phndModule:*mut HMODULE,) -> HRESULT,
    fn GetProcAddress(pszProcName: LPCSTR, ppProc: *mut LPVOID,) -> HRESULT,
    fn GetInterface(rclsid:REFCLSID, riid:REFIID, ppUnk:*mut LPVOID,) -> HRESULT,
    fn IsLoadable(pbLoadable: *mut BOOL,) -> HRESULT,
    fn SetDefaultStartupFlags(dwStartupFlags: DWORD, pwzHostConfigFile: LPCWSTR,) -> HRESULT,
    fn GetDefaultStartupFlags(pdwStartupFlags:*mut DWORD, pwzHostConfigFile:LPWSTR,
        pcchHostConfigFile:*mut DWORD,) -> HRESULT,
    fn BindAsLegacyV2Runtime() -> HRESULT,
    fn IsStarted(pbStarted:*mut BOOL, pdwStartupFlags: *mut DWORD,) -> HRESULT,
});

RIDL!(#[uuid(0x90F1A06C, 0x7712, 0x4762, 0x86, 0xB5, 0x7A, 0x5E, 0xBA, 0x6B, 0xDB, 0x02)]
interface ICLRRuntimeHost(ICLRRuntimeHostVtbl): IUnknown(IUnknownVtbl) {
    fn Start() -> HRESULT,
    fn Stop() -> HRESULT,
    fn SetHostControl(pHostControl: *mut c_void /*IHostControl*/,) -> HRESULT,
    fn GetHostControl(pHostControl: *mut *mut c_void /*IHostControl*/,) -> HRESULT,
    fn UnloadAppDomain(dwAppDomainId:DWORD, fWaitUntilDone: BOOL,) -> HRESULT,
    fn ExecuteInAppDomain(dwAppDomainId:DWORD,
        pCallback: *mut c_void /*FExecuteInAppDomainCallback*/, cookie: LPVOID,) -> HRESULT,
    fn GetCurrentAppDomainId(pdwAppDomainId: *mut DWORD,) -> HRESULT,
    fn ExecuteApplication(pwzAppFullName: LPCWSTR, dwManifestPaths:DWORD,
        ppwzManifestPaths: *mut LPCWSTR, dwActivationData: DWORD, ppwzActivationData: *mut LPCWSTR,
        pReturnValue: *mut i32,) -> HRESULT,
    fn ExecuteInDefaultAppDomain(pwzAssemblyPath:LPCWSTR, pwzTypeName:LPCWSTR,
        pwzMethodName:LPCWSTR, pwzArgument:LPCWSTR, pReturnValue: *mut DWORD,) -> HRESULT,
});

type CLRCreateInstanceFn = unsafe extern "stdcall" fn(
    clsid: REFCLSID,
    riid: REFIID,
    ppInterface: *mut *mut ICLRMetaHost,
) -> HRESULT;

fn to_wchar(str: &str) -> Vec<u16> {
    OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect()
}

fn load_library(name: &str) -> Result<HMODULE, Box<dyn std::error::Error>> {
    let handle = unsafe { LoadLibraryW(to_wchar(name).as_ptr()) };
    if handle == null_mut() {
        panic!("Failed to LoadLibrary");
    }

    Ok(handle)
}

pub fn get_proc_address(
    handle: HMODULE,
    name: &str,
) -> Result<FARPROC, Box<dyn std::error::Error>> {
    let addr = unsafe { GetProcAddress(handle, CString::new(name).unwrap().into_raw()) };
    if addr == null_mut() {
        panic!("GetProcAddress: {} not found in module!", name);
    }

    return Ok(addr);
}

fn initialize_clr() -> Result<(), Box<dyn std::error::Error>> {
    let meta_host: *mut ICLRMetaHost = unsafe {
        let mut meta_host: *mut ICLRMetaHost = null_mut();

        let handle = load_library("mscoree.dll")?;
        let clr_create_instance = get_proc_address(handle, "CLRCreateInstance")?;
        let clr_create_instance_fn: CLRCreateInstanceFn = std::mem::transmute(clr_create_instance);
        let hr =
            (clr_create_instance_fn)(&CLSID_CLR_META_HOST, &IID_ICLR_META_HOST, &mut meta_host);
        if hr != 0 {
            panic!("Failed to create MetaHost!");
        }

        if meta_host == null_mut() {
            panic!("MetaHost instance is null!");
        }

        meta_host
    };

    let runtime_info = unsafe {
        let mut runtime_info: *mut ICLRRuntimeInfo = null_mut();

        let hr = (*meta_host).GetRuntime(
            to_wchar("v4.0.30319").as_ptr(),
            &IID_ICLR_RUNTIME_INFO,
            &mut runtime_info,
        );
        if hr != 0 {
            panic!("Failed to create runtime!");
        }

        if runtime_info == null_mut() {
            panic!("Runtime instance is null!");
        }

        let mut loadable: BOOL = FALSE;
        let hr = (*runtime_info).IsLoadable(&mut loadable);
        if hr != 0 {
            panic!("Failed to check loadability!");
        }

        if loadable == FALSE {
            panic!("Runtime is not loadable!");
        }

        runtime_info
    };

    let runtime_host: *mut ICLRRuntimeHost = unsafe {
        let mut runtime_host: *mut c_void = null_mut();
        let hr = (*runtime_info).GetInterface(
            &CLSID_CLR_RUNTIME_HOST,
            &IID_ICLR_RUNTIME_HOST,
            &mut runtime_host,
        );

        if hr != 0 {
            panic!("Failed to query runtime host!");
        }

        if runtime_host == null_mut() {
            panic!("Runtime host instance is null!");
        }

        std::mem::transmute(runtime_host)
    };

    // Initialize CLR
    unsafe {
        let hr = (*runtime_host).Start();
        if hr != 0 {
            panic!("Failed to start CLR! HRESULT: {}", hr);
        }
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing CLR...");

    // Initialize CLR host in process
    initialize_clr()?;

    println!("CLR initialized.");

    // Wait for user input
    use std::io::Read;
    std::io::stdin().read(&mut [0])?;

    Ok(())
}
