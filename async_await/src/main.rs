use std::time::Duration;
use tokio::time::sleep;

// we could force tokio to excute only on a single thread by
// #[tokio::main(flavor = "current_thread")]


#[tokio::main]
async fn main() {
  //my_func().await;
  
  // running concurrent tasks
  let mut handles = vec![];

  for i in 0..2{
    let handle = tokio::spawn(async move{
      my_func(i).await;
    });
    handles.push(handle);
  }

  for handle in handles{
    handle.await.unwrap();
  }
}
// rust do not have a builtin async runtime so we have to use tokio to 
// apply async tasks to the main func

async fn my_func(i:i32){
  println!("[{i}]this is aync func");
  let s1 = read_from_db().await;
  println!("[{i}]1st result: {s1}");
  let s2 = read_from_db().await;
  println!("[{i}]2nd result: {s2}");
}

async fn read_from_db() -> String{
  sleep(Duration::from_millis(50)).await;
  // the tokio sleep func stops the func only instead of the whole thread
  "DB result".to_owned()
}


// async is the syntax sugar for the implimentation of the trait Future
/*
fn my_func() -> impl Future<Output = ()>{
  println!("its a async func");
}
*/
// futures are similar to promises of js except they are lazy, this makes futures
// a zero cost abstraction

// output
/*
[0]this is aync func
[1]this is aync func
[1]1st result: DB result
[0]1st result: DB result
[1]2nd result: DB result
[0]2nd result: DB result                                                                                 [0]2nd result: DB result
*/