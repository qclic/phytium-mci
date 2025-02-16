use super::mci_cid::MCICid;
use super::mci_csd::MCICsd;
use super::mci_scr::MCIScr;
use super::mci_status::MCIStatus;
pub struct SdReg {
    pub ocr: u32,
    pub cid: MCICid,
    pub rca: u32,
    pub csd: MCICsd,
    pub scr: MCIScr,
    pub status: MCIStatus,
}

impl SdReg {
    pub fn new() -> SdReg {
        SdReg {
            ocr: 0,
            cid: MCICid::default(),
            rca: 0,
            csd: MCICsd::default(),
            scr: MCIScr::default(),
            status: MCIStatus::default(),
        }
    }
}