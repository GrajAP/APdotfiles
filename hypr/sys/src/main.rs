use sysinfo::{CpuExt, System, SystemExt};
fn main() {

let mut sys = System::new_all();
println!("  {}GB/{}GB", sys.used_memory()*95367432/100000000000000000, sys.total_memory()*95367432/100000000000000000);
sys.refresh_cpu(); // Refreshing CPU information.
    print!("  {}%",sys.cpus()[0].cpu_usage()); 
}
