use spec::{LinkArgs, LinkerFlavor, Target, TargetOptions, TargetResult, RelroLevel, PanicStrategy};

pub fn target() -> TargetResult {
    Ok(Target {
        llvm_target: "aarch64-unknown-linux-gnu".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        target_c_int_width: "32".to_string(),
        target_env: "trustzone".to_string(),
        data_layout: "e-m:e-i8:8:32-i16:16:32-i64:64-i128:128-n32:64-S128".to_string(),
        arch: "aarch64".to_string(),
        target_os: "optee".to_string(),
        target_vendor: "unknown".to_string(),
        linker_flavor: LinkerFlavor::Ld,
        options: TargetOptions {
            executables: true,
            abi_blacklist: super::arm_base::abi_blacklist(),
            dynamic_linking: false,
            has_rpath: true,
            linker_is_gnu: true,
            max_atomic_width: Some(128),
            panic_strategy: PanicStrategy::Abort,
            position_independent_executables: true,
            relro_level: RelroLevel::Full,
            .. Default::default()
        },
    })
}
