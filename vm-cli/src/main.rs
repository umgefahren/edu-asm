use edu_asm_vm::memory::get_host_page_size;

fn main() {
    let host_page_size = get_host_page_size();
    println!("Host page size => {}", host_page_size);
}
