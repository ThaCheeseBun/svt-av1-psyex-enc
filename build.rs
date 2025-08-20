use std::env;

fn main() {
    if env::var("DOCS_RS").is_ok() {
        return;
    }
    let static_feature = env::var("CARGO_FEATURE_STATIC").is_ok();
    let dynamic_feature = env::var("CARGO_FEATURE_DYNAMIC").is_ok();
    if !static_feature && !dynamic_feature || static_feature && dynamic_feature {
        panic!("Please choose only one of the next features: `static` or `dynamic`")
    }
    if static_feature {
        println!("cargo:rustc-link-lib=static=SvtAv1Enc");
    }
    if dynamic_feature {
        println!("cargo:rustc-link-lib=dylib=SvtAv1Enc");
    }
    match env::var("SVT_AV1_LIB") {
        // User specify path, lookup for libraries files there
        Ok(path) => {
            println!("cargo:rustc-link-search=native={}", path);
        }
        // No user specified path, use pkg_config to find lib in system
        Err(_) => {
            let mut cfg = pkg_config::Config::new();
            cfg.exactly_version("3.0.2");
            if static_feature {
                cfg.statik(true);
            }
            let res = cfg.probe("SvtAv1Enc").map_err(|_| "Couldn't find SvtAv1Enc library, install it with your distro package manager or build it yourself and provide path to it with SVT_AV1_LIB env variable").unwrap();
            for p in res.link_files {
                println!("cargo:rustc-link-search=native={}", p.display());
            }
        }
    }
}
