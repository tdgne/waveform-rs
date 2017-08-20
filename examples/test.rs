extern crate ocl;

use ocl::ProQue;

fn main() {
    let src = r#"
__kernel void add(__global float* buffer, float scalar) {
    buffer[get_global_id(0)] += scalar;
}
"#;

    let pro_que = ProQue::builder()
    .src(src)
.dims(1 << 20)
    .build().unwrap();

    let buffer = pro_que.create_buffer::<f32>().unwrap();

    let kernel = pro_que.create_kernel("add").unwrap()
.arg_buf(&buffer)
    .arg_scl(10.0f32);

    kernel.enq().unwrap();

    let mut vec = vec![0.0f32; buffer.len()];
    buffer.read(&mut vec).enq().unwrap();

    println!("The value at index [{}] is now '{}'!", 200007, vec[200007]);
}
