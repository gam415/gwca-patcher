use std::{fs, io::Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let path = "gwca.dll";

  let mut data = fs::read(path)?;

  const SCAN_CODE_LENGTH: usize = 8;
  let scan_code: [u8; SCAN_CODE_LENGTH] = [0xf7, 0x40, 0x10, 0x00, 0x00, 0x01, 0x00, 0x74];
  const TOTAL_COUNT: usize = 3;
  let mut count = 0;
  for i in 0..(data.len() - SCAN_CODE_LENGTH) {
    let bytes = &data[i..i+SCAN_CODE_LENGTH];
    if bytes == scan_code {
      data[i+7] = 0xEB; // change JZ to JMP
      count = count + 1;
      if count >= TOTAL_COUNT {
        break;
      }
    }
  }

  if count < TOTAL_COUNT {
    return Err(format!("Only found {count}/{TOTAL_COUNT} instances of minipet target checks").into());
  }

  fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .open(path)?
    .write_all(&data)?;

  return Ok(());
}
