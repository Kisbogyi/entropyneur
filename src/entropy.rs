use std::{fs::File, io::Read};

pub fn calc_entropy(filename: &str, chunk_size: usize) -> std::io::Result<Vec<(f64, f64)>> {
    let mut file_input = File::open(filename)?;
    let mut buf = vec![];
    file_input.read_to_end(&mut buf)?;

    let buffers: Vec<&[u8]> = buf.chunks(chunk_size).collect();
    let mut outputs = vec![];
    let mut i = 0.0;
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

        outputs.push((i, output));
        i += 1.0;
    }
    Ok(outputs)
}
