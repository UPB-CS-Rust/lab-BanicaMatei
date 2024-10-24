fn main() {
    let input = [23, 82, 16, 45, 21, 94, 12, 34];
    let _i = 0;
    let mut largest = 0;
    let mut smallest = 100;
   for i in 0..7 {
    if input[i] > largest {
        largest = input[i]
    }
    if input[i] < smallest {
        smallest = input[i]
    }
   }

   println!("{}", smallest);
   println!("{}", largest);
}
