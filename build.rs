use std::env;

fn main() {
    // GENERIC_SOURCES and GENERIC_TF_SOURCES from builtins/CMakeLists.txt

    let generic_sources = [
        "absvdi2.c", "absvsi2.c", "absvti2.c", "adddf3.c", "addsf3.c", "addvdi3.c", "addvsi3.c",
        "addvti3.c", "apple_versioning.c", "ashldi3.c", "ashlti3.c", "ashrdi3.c", "ashrti3.c",
        "bswapdi2.c", "bswapsi2.c", "clzdi2.c", "clzsi2.c", "clzti2.c", "cmpdi2.c", "cmpti2.c",
        "comparedf2.c", "comparesf2.c", "ctzdi2.c", "ctzsi2.c", "ctzti2.c", "divdc3.c", "divdf3.c",
        "divdi3.c", "divmoddi4.c", "divmodsi4.c", "divmodti4.c", "divsc3.c", "divsf3.c",
        "divsi3.c", "divti3.c", "extendsfdf2.c", "extendhfsf2.c", "ffsdi2.c", "ffssi2.c",
        "ffsti2.c", "fixdfdi.c", "fixdfsi.c", "fixdfti.c", "fixsfdi.c", "fixsfsi.c", "fixsfti.c",
        "fixunsdfdi.c", "fixunsdfsi.c", "fixunsdfti.c", "fixunssfdi.c", "fixunssfsi.c",
        "fixunssfti.c", "floatdidf.c", "floatdisf.c", "floatsidf.c", "floatsisf.c", "floattidf.c",
        "floattisf.c", "floatundidf.c", "floatundisf.c", "floatunsidf.c", "floatunsisf.c",
        "floatuntidf.c", "floatuntisf.c", "fp_mode.c", "int_util.c", "lshrdi3.c", "lshrti3.c",
        "moddi3.c", "modsi3.c", "modti3.c", "muldc3.c", "muldf3.c", "muldi3.c", "mulodi4.c",
        "mulosi4.c", "muloti4.c", "mulsc3.c", "mulsf3.c", "multi3.c", "mulvdi3.c", "mulvsi3.c",
        "mulvti3.c", "negdf2.c", "negdi2.c", "negsf2.c", "negti2.c", "negvdi2.c", "negvsi2.c",
        "negvti2.c", "os_version_check.c", "paritydi2.c", "paritysi2.c", "parityti2.c",
        "popcountdi2.c", "popcountsi2.c", "popcountti2.c", "powidf2.c", "powisf2.c", "subdf3.c",
        "subsf3.c", "subvdi3.c", "subvsi3.c", "subvti3.c", "trampoline_setup.c", "truncdfhf2.c",
        "truncdfsf2.c", "truncsfhf2.c", "ucmpdi2.c", "ucmpti2.c", "udivdi3.c", "udivmoddi4.c",
        "udivmodsi4.c", "udivmodti4.c", "udivsi3.c", "udivti3.c", "umoddi3.c", "umodsi3.c",
        "umodti3.c",
    ];

    let generic_tf_sources = [
        "addtf3.c", "comparetf2.c", "divtc3.c", "divtf3.c", "extenddftf2.c", "extendhftf2.c",
        "extendsftf2.c", "fixtfdi.c", "fixtfsi.c", "fixtfti.c", "fixunstfdi.c", "fixunstfsi.c",
        "fixunstfti.c", "floatditf.c", "floatsitf.c", "floattitf.c", "floatunditf.c",
        "floatunsitf.c", "floatuntitf.c", "multc3.c", "multf3.c", "powitf2.c", "subtf3.c",
        "trunctfdf2.c", "trunctfhf2.c", "trunctfsf2.c",
    ];

    let sources = generic_sources
        .iter()
        .chain(generic_tf_sources.iter())
        .map(|f| format!("builtins/{}", f))
        .collect::<Vec<_>>();

    if env::var("TARGET").map_or(false, |t| t.starts_with("wasm")) {
        // Apple clang doesn't support wasm32, so use Homebrew clang by default.
        if env::var("HOST") == Ok("x86_64-apple-darwin".to_string()) {
            if env::var("CC").is_err() {
                std::env::set_var("CC", "/usr/local/opt/llvm/bin/clang");
            }
            if env::var("AR").is_err() {
                std::env::set_var("AR", "/usr/local/opt/llvm/bin/llvm-ar");
            }
        } else if env::var("HOST") == Ok("aarch64-apple-darwin".to_string()) {
            if env::var("CC").is_err() {
                std::env::set_var("CC", "/opt/homebrew/opt/llvm/bin/clang");
            }
            if env::var("AR").is_err() {
                std::env::set_var("AR", "/opt/homebrew/opt/llvm/bin/llvm-ar");
            }
        }
    }

    cc::Build::new().files(sources).compile("compiler-rt");
}
