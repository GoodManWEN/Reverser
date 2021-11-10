#[macro_export]
macro_rules! printf {
    () => {println!("")};
    ($( $args: expr ),*) => {
        $(
            print!("{:?} " , $args);
        )* 
        print!("\n");
    };
}