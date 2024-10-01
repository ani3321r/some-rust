// macro rule of vec macro
#[macro_export]
macro_rules! vec{
    ( $( $x:expr ),* ) => { // a match expression
        {
            let mut temp_vec = Vec::new();
            $(
              temp_vec.push($x);  
            )*
            temp_vec
        }
    };
}
