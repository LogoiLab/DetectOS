mod mac;
mod win;
mod nix;
pub mod detect{
    fn main(arch: arch, version: version, vendor: vendor) {
        struct os {
            arch: arch,
            version: version,
            vendor: vendor
        }
        impl os {
            fn new() -> os {

            }
        }
        let mut os = os.new();
        os.add_arch(arch);
        os.add_version(version);
        os.add_vendor(vendor);
    }
}
