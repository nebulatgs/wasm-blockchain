// use alkomp;

// fn main() {
//     let word: String = String::from("abc");
//     let mut init: Vec<u8> = word.into_bytes();

//     let msg_size = (init.len() * 8) as u64; // in bits

//     // Add a 1 as a delimiter
//     init.push(0x80 as u8);
//     let size: usize = (448u32 / 8u32 - init.len() as u32) as usize;

//     // Pad with zeros
//     let remaining = vec![0u8; size];
//     init.extend(&remaining);

//     // Make the last 64 bits be the size
//     let size = (msg_size).to_be_bytes();
//     init.extend(&size);

//     let mut text = Vec::new();

//     use std::convert::TryInto;
//     for i in 0..16 {
//         let val = u32::from_be_bytes(init[i * 4..(i + 1) * 4].try_into().unwrap());
//         text.push(val);
//     }

//     let hash = vec![0u32; 8];

//     let mut device = alkomp::Device::new(0);
//     let text_gpu = device.to_device(text.as_slice());
//     let hash_gpu = device.to_device(hash.as_slice());
//     let iter_gpu = device.to_device(&[0u32]);

//     let shader = wgpu::include_spirv!(env!("kernel.spv"));

//     let args = alkomp::ParamsBuilder::new()
//         .param(Some(&text_gpu))
//         .param(Some(&hash_gpu))
//         .param(Some(&iter_gpu))
//         .build(Some(0));

//     let compute = device.compile("main_cs", &shader, &args.0).unwrap();

//     device.call(compute, (1, 1, 1), &args.1);

//     //let text_res = futures::executor::block_on(device.get(&text_gpu)).unwrap();
//     //let text_res = &collatz[0..text.len()];

//     let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
//     let hash_res = &hash_res[0..hash.len()];

//     //println!("{:?}", hash_res);
//     //for h in 0..hash_res.len() {
//     //    println!("{:#034b}", hash_res[h]);
//     //}
//     let result: String = hash_res.into_iter().map(|x| format!("{:x}", x)).collect();
//     //println!("{}", result);
//     assert_eq!(
//         result,
//         "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
//     );
// }
