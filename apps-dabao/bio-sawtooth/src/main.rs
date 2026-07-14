use bao1x_api::bio::{BioApi, ClockMode, CoreConfig, IoConfig, IoConfigMode};
use bao1x_api::bio_code;
use bao1x_api::bio_resources::{BioResources, ResourceSpec};
use bao1x_hal::bio::Bio;

const OUTPUT_MASK: u32 = 0b0001_1100_0000_1111_0000_0000_0011_1110;

const QUANTUM_DIVIDER: u16 = 40;
const QUANTUM_FRACTION: u8 = 0;

#[rustfmt::skip]
bio_code!(
    sawtooth_bio,
    SAWTOOTH_BIO_START,
    SAWTOOTH_BIO_END,
    "li x1, 0b00011100000011110000000000111110",
    "mv x26, x1",
    "mv x24, x1",
    "li x2, 0",
    "li x3, 0b00000100000000000000000000000000",
    "li x4, 0b00000000000011110000000000111110",
    "li x5, 0b00000000000000010000000000000000",
    "li x6, 0b00000000000000000000000000111110",

    "10:",
    "mv x20, x0",

    "mv x21, x2",

    "beq x2, x1, 22f",

    "and x7, x2, x4",
    "beq x7, x4, 23f",

    "and x7, x2, x6",
    "beq x7, x6, 24f",

    "addi x2, x2, 0b10",
    "j 10b",

    "22:",
    "li x2, 0",
    "j 10b",

    "23:",
    "xor x2, x2, x4",
    "add x2, x2, x3",
    "j 10b",

    "24:",
    "xor x2, x2, x6",
    "add x2, x2, x5",
    "j 10b"
);

fn main() -> ! {
    log_server::init_wait().unwrap();
    log::set_max_level(log::LevelFilter::Info);

    let mut bio = Bio::new();

    let resource_spec = ResourceSpec::new("sawtooth_bio").any_core().pins_from_mask(OUTPUT_MASK);

    let resource_grant = bio.claim_resources(&resource_spec).expect("resource error");

    let core = resource_grant.cores[0];

    let core_config = CoreConfig { clock_mode: ClockMode::FixedDivider(QUANTUM_DIVIDER, QUANTUM_FRACTION) };

    let actual_quantum_rate =
        bio.init_core(core, sawtooth_bio(), core_config).expect("failed to initialize BIO program");

    let io_config = IoConfig {
        mode: IoConfigMode::SetOnly,
        mapped: OUTPUT_MASK,
        sync_bypass: 0,
        oe_inv: 0,
        o_inv: 0,
        i_inv: 0,
        snap_inputs: None,
        snap_outputs: None,
    };

    bio.setup_io_config(io_config).expect("failed to configure BIO I/O");

    bio.set_core_run_state(&resource_grant, true);

    match actual_quantum_rate {
        Some(rate_hz) => {
            log::info!("BIO sawtooth started: core={:?}, quantum={} Hz", core, rate_hz,);
        }
        None => {
            log::warn!("BIO sawtooth started, but the quantum rate was not reported: core={:?}", core);
        }
    }

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}
