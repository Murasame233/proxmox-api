use serde::{Deserialize, Serialize};

/// A MAC address.
///
/// `B` represents whether this address is a unicast address, or if it may be unicast
/// _or_ multicast.
///
/// If `B` is `false`, this is a unicast address. If `B` is `true` this is address
/// may represent either a unicast _or_ a multicast address.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "&str", into = "String")]
pub struct MacAddr<const B: bool>([u8; 6]);

impl MacAddr<false> {
    pub fn try_new(addr: [u8; 6]) -> Option<Self> {
        let me = Self(addr);

        (!me.ig_bit_int()).then_some(me)
    }
}

impl TryFrom<MacAddr<true>> for MacAddr<false> {
    type Error = ();

    fn try_from(value: MacAddr<true>) -> Result<Self, Self::Error> {
        MacAddr::<false>::try_new(value.0).ok_or(())
    }
}

impl From<MacAddr<false>> for MacAddr<true> {
    fn from(value: MacAddr<false>) -> Self {
        Self(value.0)
    }
}

impl MacAddr<true> {
    pub fn new(addr: [u8; 6]) -> Self {
        Self(addr)
    }
}

impl<const B: bool> MacAddr<B> {
    pub fn get(&self) -> [u8; 6] {
        self.0
    }

    fn ig_bit_int(&self) -> bool {
        (self.0[0] & 0x80) == 0x80
    }

    pub fn ig_bit(&self) -> bool {
        // Gotta make sure we actually uphold this...
        debug_assert!(self.ig_bit_int());
        B
    }
}

impl<const B: bool> std::fmt::Display for MacAddr<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5]
        )
    }
}

impl<const B: bool> From<MacAddr<B>> for String {
    fn from(value: MacAddr<B>) -> Self {
        format!("{value}")
    }
}

impl<const B: bool> TryFrom<&str> for MacAddr<B> {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (count, addr) =
            value
                .split(':')
                .try_fold((0usize, [0u8; 6]), |(mut idx, mut accum), input| {
                    if idx == 6 {
                        Err("Too many segments in MAC address".to_string())
                    } else if let Ok(value) = u8::from_str_radix(input, 16) {
                        accum[idx] = value;
                        idx += 1;
                        Ok((idx, accum))
                    } else {
                        Err(format!("Invalid mac address segment: {input}"))
                    }
                })?;

        let addr = MacAddr(addr);

        if count != 6 {
            Err(format!(
                "Not enough segments in MAC address. Expected 6, got {count}"
            ))
        } else if !B && addr.ig_bit_int() {
            Err(format!(
                "MAC address type disallows IG bit, but it is set. Address: {addr}"
            ))
        } else {
            Ok(addr)
        }
    }
}
