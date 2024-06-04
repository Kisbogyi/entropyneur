use std::{fs::File, io::Read};

fn main() -> std::io::Result<()> {
    let chunk_size = 128;
    let filename = "/usr/bin/ls";

    let mut file_input = File::open(filename)?;
    let mut buf = vec![];
    file_input.read_to_end(&mut buf)?;

    let buffers: Vec<&[u8]> = buf.chunks(chunk_size).collect();
    for buffer in buffers {
        let mut bytes = vec![0; 256];
        for value in buffer.to_vec().into_iter() {
            bytes[usize::from(value)] += 1;
        }

        let summa: u16 = bytes.iter().sum();
        let mut p = bytes
            .iter()
            .map(|&item| f64::from(item) / f64::from(summa))
            .collect::<Vec<_>>();

        p.retain(|&item| item != 0.0);

        let output = p
            .iter()
            .fold(0.0, |entropy, &item| entropy - item * item.log(256.0));

        println!("{}", output);
    }
    Ok(())
}
