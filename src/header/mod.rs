pub mod android_header;
pub mod samsung_header;

pub use self::android_header::{AospHeader0, AospHeader1, AospHeader2, AospHeader3};
pub use self::samsung_header::SamsungHeader as Header;
pub use self::samsung_header::SamsungHeader;
pub use self::samsung_header::SAMSUNG_HEADER_SIZE;

pub enum HeaderKind {
    Samsung(SamsungHeader),
    Aosp0(AospHeader0),
    Aosp1(AospHeader1),
    Aosp2(AospHeader2),
    Aosp3(AospHeader3),
}
