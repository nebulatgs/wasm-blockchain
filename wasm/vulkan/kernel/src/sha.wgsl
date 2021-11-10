[[block]] struct IntArr {
    arr: [[stride(4)]] array<u32>;
};

[[group(0), binding(0)]]
var<storage> text: IntArr;
[[group(0), binding(1)]]
var<storage, read_write> hash: IntArr;
[[group(0), binding(2)]]
var<storage> iter: IntArr;


// Rotation right: u32.rotate_right(n: u32)
fn rot_r(x: u32, n: u32) -> u32 {
    return x >> n | (x << (32u - n));
}

fn Sigma0(x: u32) -> u32 {
    return rot_r(x, 2u) ^ rot_r(x, 13u) ^ rot_r(x, 22u);
    //x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}


fn Sigma1(x: u32) -> u32 {
    return rot_r(x, 6u) ^ rot_r(x, 11u) ^ rot_r(x, 25u);
    //x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

fn sigma0(x: u32) -> u32 {
    return rot_r(x, 7u) ^ rot_r(x, 18u) ^ (x >> 3u);
    //x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

fn sigma1(x: u32) -> u32 {
    return rot_r(x, 17u) ^ rot_r(x, 19u) ^ (x >> 10u);
    //x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}
// Choice operation
fn ch(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) ^ (u32(!bool(x)) & z);
}


// Majority operation
fn maj(x: u32, y: u32, z: u32) -> u32 {
    return (x & y) ^ (x & z) ^ (y & z);
}

var<private> K: array<u32, 64> = array<u32, 64>(
    0x428a2f98u, 0x71374491u, 0xb5c0fbcfu, 0xe9b5dba5u, 0x3956c25bu, 0x59f111f1u, 0x923f82a4u, 0xab1c5ed5u,
    0xd807aa98u, 0x12835b01u, 0x243185beu, 0x550c7dc3u, 0x72be5d74u, 0x80deb1feu, 0x9bdc06a7u, 0xc19bf174u,
    0xe49b69c1u, 0xefbe4786u, 0x0fc19dc6u, 0x240ca1ccu, 0x2de92c6fu, 0x4a7484aau, 0x5cb0a9dcu, 0x76f988dau,
    0x983e5152u, 0xa831c66du, 0xb00327c8u, 0xbf597fc7u, 0xc6e00bf3u, 0xd5a79147u, 0x06ca6351u, 0x14292967u,
    0x27b70a85u, 0x2e1b2138u, 0x4d2c6dfcu, 0x53380d13u, 0x650a7354u, 0x766a0abbu, 0x81c2c92eu, 0x92722c85u,
    0xa2bfe8a1u, 0xa81a664bu, 0xc24b8b70u, 0xc76c51a3u, 0xd192e819u, 0xd6990624u, 0xf40e3585u, 0x106aa070u,
    0x19a4c116u, 0x1e376c08u, 0x2748774cu, 0x34b0bcb5u, 0x391c0cb3u, 0x4ed8aa4au, 0x5b9cca4fu, 0x682e6ff3u,
    0x748f82eeu, 0x78a5636fu, 0x84c87814u, 0x8cc70208u, 0x90befffau, 0xa4506cebu, 0xbef9a3f7u, 0xc67178f2u,
);

let INIT_HASH: array<u32, 8> = array<u32, 8>(
    0x6a09e667u, 0xbb67ae85u, 0x3c6ef372u, 0xa54ff53au, 0x510e527fu, 0x9b05688cu, 0x1f83d9abu, 0x5be0cd19u,
);

fn hash_fn(x: u32, offset: u32, iter: u32) {
    // Offsets for loading and storing in data buffers
    let load_offset = (iter + x + offset) * 16u;
    let store_offset = x * 8u;
var a: u32;
var b: u32;
var c: u32;
var d: u32;
var e: u32;
var f: u32;
var g: u32;
var h: u32;
var t1: u32;
var t2: u32;
    // var (a, b, c, d, e, f, g, h, t1, t2): (
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    //     u32,
    // );

    // Need to manually unroll declaration
    var m = array<u32, 64> (
        0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u,
        0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u,
        0u, 0u, 0u, 0u,
    );

    // Create the message schedule
    // The first 16 are assumed to be given
    for (var i: i32 = 0; i < 16; i = i + 1)
    {
        m[i] = text.arr[load_offset + u32(i)];
    }

    // Compute the remaining message schedule
    for (var i: i32 = 16; i < 64; i = i + 1) {
        m[i] = sigma1(m[i - 2]) + m[i - 7] + sigma0(m[i - 15]) + m[i - 16];
        //println!("{} {:#034b}", i, m[i]);
    }

    // Do compression
    // The initial hash value as sqrt of primes
    if (iter == 0u) {
        a = INIT_HASH[0u];
        b = INIT_HASH[1u];
        c = INIT_HASH[2u];
        d = INIT_HASH[3u];
        e = INIT_HASH[4u];
        f = INIT_HASH[5u];
        g = INIT_HASH[6u];
        h = INIT_HASH[7u];
    } else {
        a = hash.arr[store_offset + 0u];
        b = hash.arr[store_offset + 1u];
        c = hash.arr[store_offset + 2u];
        d = hash.arr[store_offset + 3u];
        e = hash.arr[store_offset + 4u];
        f = hash.arr[store_offset + 5u];
        g = hash.arr[store_offset + 6u];
        h = hash.arr[store_offset + 7u];
    }

    for (var i: i32 = 0; i < 64; i = i + 1) {
        t1 = Sigma1(e) + ch(e, f, g) + h + K[i] + m[i];
        t2 = Sigma0(a) + maj(a, b, c);
        h = g;
        g = f;
        f = e;
        e = d + t1;
        d = c;
        c = b;
        b = a;
        a = t1 + t2;
    }

    // Add the original hashed message with initial hash
    if (iter == 0u) {
        a = a + INIT_HASH[0u];
        b = b + INIT_HASH[1u];
        c = c + INIT_HASH[2u];
        d = d + INIT_HASH[3u];
        e = e + INIT_HASH[4u];
        f = f + INIT_HASH[5u];
        g = g + INIT_HASH[6u];
        h = h + INIT_HASH[7u];
    } else {
        a = a + hash.arr[store_offset + 0u];
        b = b + hash.arr[store_offset + 1u];
        c = c + hash.arr[store_offset + 2u];
        d = d + hash.arr[store_offset + 3u];
        e = e + hash.arr[store_offset + 4u];
        f = f + hash.arr[store_offset + 5u];
        g = g + hash.arr[store_offset + 6u];
        h = h + hash.arr[store_offset + 7u];
    }

    // Store result
    hash.arr[store_offset + 0u] = a;
    hash.arr[store_offset + 1u] = b;
    hash.arr[store_offset + 2u] = c;
    hash.arr[store_offset + 3u] = d;
    hash.arr[store_offset + 4u] = e;
    hash.arr[store_offset + 5u] = f;
    hash.arr[store_offset + 6u] = g;
    hash.arr[store_offset + 7u] = h;
}

// #[test]
// fn test_hash_fn() {
//     let word: String = String::from("abc");
//     let mut init: Vec<u8> = word.into_bytes();

//     let msg_size = (init.len() * 8) as u32; // in bits

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

//     let mut hash = vec![0u32; 8];

//     hash_fn(text.as_slice(), hash.as_mut_slice(), 0, 0, 0);

//     let result: String = hash.into_iter().map(|x| format!("{:x}", x)).collect();
//     assert_eq!(
//         result,
//         "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
//     );
// }

[[stage(compute), workgroup_size(64)]]
fn main_cs(
    [[builtin(global_invocation_id)]] gid: vec3<u32>,
) {
    // The sha specification loops in blocks of 512
    let num_loops = iter.arr[gid.x];

    // Calculate where the memory offset for each kernel instance
    // which depends upon the number of iterations required by all previous
    // kernel invocations
    var offset = 0u;
    for (var i: i32 = 0; i < i32(gid.x); i = i + 1) {
        offset = offset + iter.arr[i] - 1u;
    }

    for (var i: i32 = 0; i < i32(num_loops); i = i + 1) {
        hash_fn(gid.x, offset, u32(i));
    }
}
