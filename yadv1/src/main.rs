fn main() {

    let v = 1_024 + 0xff +0o77 + 0b1111_1111; //1024+255+63+255 = 1597
   assert!(v == 1597);
   println!("{}", v);

   let x: i32 =5;
   let y: i32;

   y = x;

   println!("{}",y);
   let z = 10;
   println!("{}", z);
   println!("Degmaster");


       let x1:f64 = 1_000.00_1;
       let y1:f32 = 0.12;
       let z1:f64 = 0.01_f64;
       println!("{}", x1);
       println!("{}", y1);
       println!("{}", z1);

       let xx:f64 = x1;


       assert_eq!(type_of(&xx), "f64".to_string());
       // Print the return value of the function
       println!("The return value of type_of(&xx) is: {}", type_of(&xx));

       assert!(0.1_f32 + 0.2_f32 == 0.3_f32);
       assert!(0.1 as f32 + 0.2 as f32 == 0.3 as f32);
       println!("Success 0.1_f32 or 0.1 as f32");

   fn type_of<T>(_: &T) -> String {
       format!("{}", std::any::type_name::<T>())
   }
       println!("Success!")
  
}
