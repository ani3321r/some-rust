use std::thread;
use std::time::Duration;

// a expensive func 
fn simulated_exp_calc(intensity: u32) -> u32{
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
} // as calling this func many times require more time we use closures

fn main() {
    let sim_intensity = 10;
    let sim_rand_num = 7;

    gen_workout(sim_intensity, sim_rand_num);


    // closure captures val from env in three ways
    // 1.taking ownership 2. borrowing mutably 3. borrowing immutably
    // FnOnce, FnMut, Fn

    let x = vec![1, 2, 3];
    let eq_to_x = |z| z == x; // do not take ownership
    // let eq_to_x = move |z| z == x; // take ownership
    println!("cant use x here {:?}", x);
    let y = vec![1, 2, 3];
    assert!(eq_to_x(y));
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calc: T,
    val: Option<u32>,
}    

impl<T> Cacher<T>
where 
    T: Fn(u32) -> u32,
{
    fn new(calc: T) -> Cacher<T>{
        Cacher{
            calc,
            val: None,
        }
    }

    fn val(&mut self, arg: u32) -> u32{
        match self.val{
            Some(v) => v,
            None =>{
                let v = (self.calc)(arg);
                self.val = Some(v);
                v
            }
        }
    }
}

fn gen_workout(intensity: u32, rand_num: u32){
    let mut cached_res = Cacher::new(|num|{
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    }); //we don't have to annotate the types of a closure

    let example_closure = |x| x;

    let s =example_closure(String::from("hello"));
    // let n = example_closure(5); //this is not allowed as it is pre-annotated with string


    if intensity < 25{
        println!("today's pushup {}",
        cached_res.val(intensity)
    );    
        println!("situps {}",
        cached_res.val(intensity)
    );
    } else {
        if rand_num == 3{
            println!("take a break");
        } else {
            println!(
                "run for {} mins",
                cached_res.val(intensity)
            );
        }
    }
}