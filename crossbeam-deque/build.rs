fn main() {
    if cfg!(feature = "use-hazptr") {
        std::env::set_var("HAZPTR_SCAN_THRESHOLD", "4");
    } else {
        std::env::set_var("DEBRA_EPOCH_CACHE_SIZE", "4");
        std::env::set_var("DEBRA_CHECK_THRESHOLD", "128");
        std::env::set_var("DEBRA_ADVANCE_THRESHOLD", "0");
    }
}
