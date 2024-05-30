use anyhow::Result;
use bytes::{BufMut, BytesMut};

fn main() -> Result<()> {
    let mut buf = BytesMut::with_capacity(1024);
    buf.extend_from_slice(b"hello world\n");
    buf.put(&b"goodbye world"[..]);
    buf.put_i64(0xdeadbeef);

    println!("{:?}", buf);
    let a = buf.split();
    let mut b = a.freeze();

    // let pos = b.binary_search(&10).unwrap();
    let pos = b.iter().position(|&b| b == b'\n').unwrap();
    let c = b.split_to(pos + 1);
    println!("c: {:?}", c);
    println!("b: {:?}", b);

    let subset = b.slice(..);
    let subset = subset.as_ref();
    // let subset = .as_ref();
    let d = b.slice_ref(subset);
    println!("d: {:?}", d);

    Ok(())
}
