fn os_detect(){
    if cfg!(windows){
        println!("Running on Windows");
    }
    else if cfg!(linux) {
        println!("Running on Linux")
    }
}

fn main() {
    println!("Welcome to HAC");
    println!("HAC is a WIP CS:GO Anticheat client")
    os_detect()

    

}
