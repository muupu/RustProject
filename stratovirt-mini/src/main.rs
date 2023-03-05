use kvm_bindings::kvm_userspace_memory_region;
use kvm_ioctls::{Kvm, VcpuExit};

fn main() {
    println!("Hello, world113!");

    let mem_size = 0x10000;
    let guest_addr = 0x1000;

    let asm_code: &[u8] = &[
        0xba, 0xf8, 0x03,   // mov $0x3f8, %dx  将0x3f8存在寄存器DX中
        0x00, 0xd8,         // add %bl, %al     将AL寄存器和BL寄存器中的值相加,输出到0x3f8端口
        0x04, b'0',         // add $'0', %al
        0xee,               // out %al, (%dx)   
        0xb0, b'\n',        // mov $'\n', %al   将换行符“\n”输出到0x3f8端口
        0xee,               // out %al, (%dx)
        0xf4,               // hlt              执行HLT指令停止虚拟机的运行
    ];
}
