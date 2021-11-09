pub mod runner;

// use std::env;
// use crate::runner::prepare_for_gpu;
// Prepare the text data for GPU by padding the bits to multiples of 512
// - Append 1 as a delimiter
// - Append 0s
// - The last 64 bits denote the size of original message

// fn main() {
//     let words: Vec<String> = env::args().skip(1).collect();
//     let count = words.len();
//     if count == 0 {
//         println!("Input a list of strings to hash");
//         return;
//     }

//     // A Vec of bit strings, and a vec of "number of iterations"
//     let (texts, sizes): (Vec<Vec<u32>>, Vec<u32>) =
//         words.into_iter().map(|x| prepare_for_gpu(x)).unzip();

//     let texts: Vec<u32> = texts.into_iter().flatten().collect();

//     let hash = vec![0u32; count * 8];

//     // Check number of bits
//     assert_eq!(hash.len() * core::mem::size_of::<u32>() * 8, 8 * 32 * count);

//     let mut device = runner::Device::new(0);
//     let text_gpu = device.to_device(texts.as_slice());
//     let hash_gpu = device.to_device(hash.as_slice());
//     let size_gpu = device.to_device(sizes.as_slice());

//     let shader = wgpu::include_spirv!("/home/coder/wasm-blockchain/wasm/vulkan/target/wasm32-unknown-unknown/spirv-builder/spirv-unknown-unknown/release/kernel.spv");

//     let args = runner::ParamsBuilder::new()
//         .param(Some(&text_gpu))
//         .param(Some(&hash_gpu))
//         .param(Some(&size_gpu))
//         .build(Some(0));

//     let compute = device.compile("main_cs", &shader, &args.0).unwrap();

//     device.call(compute, (count as u32, 1, 1), &args.1);

//     let hash_res = futures::executor::block_on(device.get(&hash_gpu)).unwrap();
//     let hash_res = &hash_res[0..hash.len()];

//     let result: String = hash_res.into_iter().map(|x| format!("{:08x}", x)).collect();
//     let chunks = result
//         .as_bytes()
//         .chunks(64)
//         .map(std::str::from_utf8)
//         .collect::<Result<Vec<&str>, _>>()
//         .unwrap();
//     for c in chunks {
//         println!("{}", c);
//     }
// }
