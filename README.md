# Rust CLR Loader

This is a native Rust implementation of the [CLR hosting interfaces](https://learn.microsoft.com/en-us/dotnet/framework/unmanaged-api/hosting/clr-hosting-interfaces), in this case, only `ICLRMetaHost`, `ICLRRuntimeInfo`, `ICLRRuntimeHost` and `CLRCreateInstance` are imported.

This is _Offensive Rust_ material, you could use this to inject managed code into an unmanaged process.

From [Injecting .NET Assembly to an Unmanaged Process](https://www.ired.team/offensive-security/code-injection-process-injection/injecting-and-executing-.net-assemblies-to-unmanaged-process)

At a high level, it works as follows:

- `CLRCreateInstance` is used to retrieve an interface `ICLRMetaHost`
- `ICLRMetaHost->GetRuntime` is used to retrieve `ICLRRuntimeInfo` interface for a specified CLR version
- `ICLRRuntimeInfo->GetInterface` is used to load the CLR into the current process and retrieve an interface `ICLRRuntimeHost`
- `ICLRRuntimeHost->Start` is used to initialize the CLR into the current process

At this point, managed shellcode can be injected into the CLR-loaded process and executed.

## Compiling

```bash
cargo build --releaseg
```

## Credits

- [Injecting .NET Assembly to an Unmanaged Process](https://www.ired.team/offensive-security/code-injection-process-injection/injecting-and-executing-.net-assemblies-to-unmanaged-process)
- [dnclr](https://github.com/jmquigs/ModelMod/tree/006e8b723ba265e2c6d77fe13db28b3b3b10024e/Native/dnclr) from [ModelMod](https://github.com/jmquigs/ModelMod)
