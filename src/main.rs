use std::{fs, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = "gwca.dll";

  let mut data = fs::read(path)?;

  const SCAN_CODE_LENGTH: usize = 5;
  let scan_code: [u8; SCAN_CODE_LENGTH] = [0x2d, 0x20, 0x00, 0x00, 0x10];
  const INNER_SCAN_CODE_LENGTH: usize = 5;
  let inner_scan_code: [u8; INNER_SCAN_CODE_LENGTH] = [0x59, 0x59, 0x84, 0xc0, 0x75];

  for i in 0..(data.len() - SCAN_CODE_LENGTH) {
    let bytes = &data[i..i+SCAN_CODE_LENGTH];
    if bytes == scan_code {
      let mut count_remaining = 2;

      for j in i..i+0x200 {
        let bytes = &data[j..j+INNER_SCAN_CODE_LENGTH];
        if bytes == inner_scan_code {
          data[j+4] = 0xEB; // change JNZ to JMP
          count_remaining = count_remaining - 1;

          if count_remaining == 0 {
            fs::OpenOptions::new()
              .write(true)
              .truncate(true)
              .open(path)?
              .write_all(&data)?;

            return Ok(());
          }
        }
      }
    }
  }

  return Err(format!("Could not find minipet target checks").into());
}
