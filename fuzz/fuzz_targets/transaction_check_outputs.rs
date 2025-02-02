#![no_main]

use libfuzzer_sys::fuzz_target;
use monero::util::fuzz_utils::{
    fuzz_create_transaction_alternative_1,
    fuzz_create_transaction_alternative_2,
    fuzz_transaction_check_outputs,
    fuzz_create_raw_extra_field,
};

fuzz_target!(|data: &[u8]| {
    let raw_extra_field = match fuzz_create_raw_extra_field(&data) {
        Ok(val) => val,
        Err(_) => {
            // This may not fail, otherwise the test cannot continue
            return;
        }
    };

    let transaction = fuzz_create_transaction_alternative_1(&data, &raw_extra_field);
    let _unused = fuzz_transaction_check_outputs(&transaction);

    let transaction = fuzz_create_transaction_alternative_2(&data, &raw_extra_field);
    let _unused = fuzz_transaction_check_outputs(&transaction);
});
