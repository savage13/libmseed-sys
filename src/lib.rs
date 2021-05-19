
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    //use std::os::raw;
    use std::ffi::CString;
    use std::ffi::CStr;
    #[test]
    fn basic() {
        let mut fp : *mut MS3FileParam = ptr::null_mut();
        let mut msr : *mut MS3Record = unsafe { msr3_init( ptr::null_mut() ) };

        let mut fpos : i64 = 0;
        let mut last : i8 = 0;
        let flags : u32 = MSF_UNPACKDATA;
        let verbose :i8 = 0;
        let mspath = CString::new("tests/multiple.seed").unwrap();


        let rv = unsafe {
            ms3_readmsr_r((&mut fp) as *mut *mut MS3FileParam,
                          (&mut msr) as *mut *mut MS3Record,
                          mspath.as_ptr(),
                          &mut fpos,
                          &mut last,
                          flags,
                          verbose)
        };
        assert_eq!(rv, MS_NOERROR as i32);
        let ms = unsafe { *msr };
        assert_eq!(ms.reclen, 512);
        assert_eq!(fpos, 0);
        assert_eq!(last, 0);
        let sid = unsafe { CStr::from_ptr(ms.sid.as_ptr()) }.to_str().unwrap();
        assert_eq!(sid, "XFDSN:IU_ANMO_00_B_H_Z");
        assert_eq!(ms.starttime, 1267252200019538000);
        assert_eq!(ms.samprate, 20.0);
        assert_eq!(ms.encoding, DE_STEIM2 as i8);
        assert_eq!(ms.pubversion, 4);
        assert_eq!(ms.samplecnt, 419);
        assert_eq!(ms.crc, 0);
        assert_eq!(ms.extralength, 33);
        assert_eq!(ms.datalength, 448);
        assert!(ms.extra != ptr::null_mut());
        assert!(ms.datasamples != ptr::null_mut());
        assert_eq!(ms.datasize, 419*4);
        assert_eq!(ms.numsamples, 419);
        assert_eq!(ms.sampletype, 'i' as std::os::raw::c_char);

                let rv = unsafe {
            ms3_readmsr_r((&mut fp) as *mut *mut MS3FileParam,
                          (&mut msr) as *mut *mut MS3Record,
                          mspath.as_ptr(),
                          &mut fpos,
                          &mut last,
                          flags,
                          verbose)
        };
        assert_eq!(rv, MS_NOERROR as i32);
        let ms = unsafe { *msr };
        assert_eq!(ms.reclen, 512);
        assert_eq!(fpos, 512);
        assert_eq!(last, 0);
        let sid = unsafe { CStr::from_ptr(ms.sid.as_ptr()) }.to_str().unwrap();
        assert_eq!(sid, "XFDSN:IU_ANMO_00_B_H_Z");
        assert_eq!(ms.starttime, 1267252220969538000);
        assert_eq!(ms.samprate, 20.0);
        assert_eq!(ms.encoding, DE_STEIM2 as i8);
        assert_eq!(ms.pubversion, 4);
        assert_eq!(ms.samplecnt, 368);
        assert_eq!(ms.crc, 0);
        assert_eq!(ms.extralength, 33);
        assert_eq!(ms.datalength, 448);
        assert!(ms.extra != ptr::null_mut());
        assert!(ms.datasamples != ptr::null_mut());
        assert_eq!(ms.datasize, 368*4);
        assert_eq!(ms.numsamples, 368);
        assert_eq!(ms.sampletype, 'i' as std::os::raw::c_char);
    }
}
