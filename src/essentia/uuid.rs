//! Pure Rust UUID v4 implementation.
#![allow(clippy::unreadable_literal, clippy::similar_names)]

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Uuid {
    bytes: [u8; 16],
}

impl Uuid {
    pub fn new_v4() -> Self {
        let mut bytes = [0u8; 16];

        // CR-163: Use canonical time module for randomness seed
        let now = essentia_core::time::unix_nanos();

        let mut rng = now as u64;

        for byte in bytes.iter_mut() {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            *byte = ((rng >> 32) & 0xFF) as u8;
        }

        // Set version to 4 (random UUID)
        bytes[6] = (bytes[6] & 0x0F) | 0x40;

        // Set variant to RFC 4122
        bytes[8] = (bytes[8] & 0x3F) | 0x80;

        Uuid { bytes }
    }

    pub fn from_bytes(bytes: [u8; 16]) -> Self {
        Uuid { bytes }
    }

    pub fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }
}

impl std::fmt::Display for Uuid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
            u32::from_be_bytes([self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3]]),
            u16::from_be_bytes([self.bytes[4], self.bytes[5]]),
            u16::from_be_bytes([self.bytes[6], self.bytes[7]]),
            u16::from_be_bytes([self.bytes[8], self.bytes[9]]),
            u64::from_be_bytes([
                self.bytes[10],
                self.bytes[11],
                self.bytes[12],
                self.bytes[13],
                self.bytes[14],
                self.bytes[15],
                0,
                0
            ]) >> 16
        )
    }
}

impl std::str::FromStr for Uuid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 36 {
            return Err("Invalid UUID length");
        }

        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 5 {
            return Err("Invalid UUID format");
        }

        let mut bytes = [0u8; 16];

        // Parse first part (8 hex chars)
        let part0 = u32::from_str_radix(parts[0], 16).map_err(|_| "Invalid hex")?;
        bytes[0..4].copy_from_slice(&part0.to_be_bytes());

        // Parse second part (4 hex chars)
        let part1 = u16::from_str_radix(parts[1], 16).map_err(|_| "Invalid hex")?;
        bytes[4..6].copy_from_slice(&part1.to_be_bytes());

        // Parse third part (4 hex chars)
        let part2 = u16::from_str_radix(parts[2], 16).map_err(|_| "Invalid hex")?;
        bytes[6..8].copy_from_slice(&part2.to_be_bytes());

        // Parse fourth part (4 hex chars)
        let part3 = u16::from_str_radix(parts[3], 16).map_err(|_| "Invalid hex")?;
        bytes[8..10].copy_from_slice(&part3.to_be_bytes());

        // Parse fifth part (12 hex chars)
        let part4 = u64::from_str_radix(parts[4], 16).map_err(|_| "Invalid hex")?;
        let part4_bytes = part4.to_be_bytes();
        bytes[10..16].copy_from_slice(&part4_bytes[2..8]);

        Ok(Uuid { bytes })
    }
}

pub fn uuid4() -> String {
    Uuid::new_v4().to_string()
}

pub fn generate_v4() -> Uuid {
    Uuid::new_v4()
}
