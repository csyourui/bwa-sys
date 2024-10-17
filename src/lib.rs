#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn aligner_example() {
        unsafe {
            let fa_path = CString::new("test/test.fasta").expect("CString::new failed");

            let ret = bwa_idx_build(
                fa_path.as_ptr(),
                fa_path.as_ptr(),
                BWTALGO_AUTO as i32,
                10000000,
            );
            if ret != 0 {
                panic!("Failed to build index");
            }
            let fa_path = CString::new("test/test.fasta").expect("CString::new failed");
            let fq_path = CString::new("test/test.fq").expect("CString::new failed");

            let idx: *mut bwaidx_t = bwa_idx_load(fa_path.as_ptr(), BWA_IDX_ALL as i32);
            if idx.is_null() {
                panic!("Failed to load index");
            }
            let arg2 = CString::new("r").expect("CString::new failed");
            let fp = gzopen(fq_path.as_ptr(), arg2.as_ptr());
            if fp.is_null() {
                panic!("Failed to open file");
            }
            let ks = kseq_init(fp);
            if ks.is_null() {
                panic!("Failed to initialize kseq");
            }
            let opt = mem_opt_init();

            let mut gold_results = vec![];
            gold_results.push(
                "FAKE-SEQ:1:FAKE-FLOWCELL-ID:1:1:0:1\t+\tNM_001039554.3\t1012\t60\t100M"
                    .to_string(),
            );
            gold_results.push(
                "FAKE-SEQ:1:FAKE-FLOWCELL-ID:1:1:0:2\t+\tNM_001039554.3\t1273\t60\t57M4D43M"
                    .to_string(),
            );
            gold_results.push(
                "FAKE-SEQ:1:FAKE-FLOWCELL-ID:1:1:0:3\t+\tNM_001039554.3\t216\t60\t100M".to_string(),
            );

            let mut seq_count = 0;
            while kseq_read(ks) >= 0 {
                let ar: mem_alnreg_v = mem_align1(
                    opt,
                    (*idx).bwt,
                    (*idx).bns,
                    (*idx).pac,
                    (*ks).seq.l as i32,
                    (*ks).seq.s,
                );
                for i in 0..ar.n {
                    let mut sam_str = String::from("");

                    let a: mem_aln_t = mem_reg2aln(
                        opt,
                        (*idx).bns,
                        (*idx).pac,
                        (*ks).seq.l as i32,
                        (*ks).seq.s,
                        ar.a.offset(i as isize),
                    );
                    let ann: *mut bntann1_t = (*(*idx).bns).anns.offset(a.rid as isize);
                    sam_str.push_str(CStr::from_ptr((*ks).name.s).to_str().unwrap());
                    sam_str.push_str("\t");
                    sam_str.push_str(vec!["+", "-"][a.is_rev() as usize]);
                    sam_str.push_str("\t");
                    sam_str.push_str(CStr::from_ptr((*ann).name).to_str().unwrap());
                    sam_str.push_str("\t");
                    sam_str.push_str(a.pos.to_string().as_str());
                    sam_str.push_str("\t");
                    sam_str.push_str(a.mapq().to_string().as_str());
                    sam_str.push_str("\t");
                    let cigar = a.cigar;
                    for j in 0..a.n_cigar {
                        let ciga_offset = *cigar.offset(j as isize);
                        sam_str.push_str(((ciga_offset >> 4) as u8).to_string().as_str());
                        sam_str
                            .push_str(vec!["M", "I", "D", "S", "H"][(ciga_offset & 0xf) as usize]);
                    }
                    println!("{:?}", sam_str);
                    assert_eq!(sam_str, gold_results[seq_count as usize]);
                }
                seq_count += 1;
            }
        }
    }
}
