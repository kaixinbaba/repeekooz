use cmd_lib::{run_fun, run_cmd};
fn main() {

    let jps_result = run_fun!(jps).unwrap();
    let mut x: Vec<&str> = jps_result.split("\n").into_iter().filter(|s| s.contains("zk36.jar")).collect::<Vec<&str>>();

    let pid: u16 = x.remove(0).replace(" zk36.jar", "").parse::<u16>().unwrap();

    println!("{:?}", pid);

    run_cmd!(kill -2 $pid);

}
