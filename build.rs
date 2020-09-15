fn main() {
    #[cfg(feature = "vendored")]
    {
        use std::env;

        // vendor if the feature is present, unless
        // BOOLECTOR_NO_VENDOR exists and isn't `0`
        if env::var("BOOLECTOR_NO_VENDOR").map_or(true, |s| s == "0") {
            let boolector = boolector_src::Build::new().prerequisites().build();
            println!("cargo:vendored=1");
            println!(
                "cargo:root={}",
                boolector.lib_dir().parent().unwrap().display()
            );
            boolector.print_cargo_metadata();

            ()
        }
    }

    println!("cargo:rustc-link-lib=boolector");
}
