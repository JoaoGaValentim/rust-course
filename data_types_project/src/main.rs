fn main() {
    /*
        Declare an i32 variable assigned to 1337.
        Use the underscore character to add a visual
        separator between the numbers.

        Cast the i32 to an i16 integer and assign the result
        to a separate variable.
    */
    let year: i32 = 1_337;
    let year: i16 = year as i16;
    let available_year: i16 = year;
    println!("The available year is {available_year}.");
    /*
        Declare a floating-point value of your choosing.
        Print out the number with 3 digits of precision.
    */
    let dollar_to_brl: f64 = 2.0 * 5.1512;
    println!("US$2 to BRL is {dollar_to_brl:.3}.");
    /*
        Declare a 'with_milk' variable set to a Boolean.
        Declare a 'with_sugar` variable set to a Boolean.

        Declare a 'is_my_type_of_coffee` variable. It should
        be set to true if the coffee has both milk and sugar.
    */
    let with_milk: bool = true;
    let with_sugar: bool = true;

    let is_my_type_of_coffee: bool = with_milk && with_sugar;
    println!("Is my type of coffee? {is_my_type_of_coffee}.");
    /*
        Declare an `is_acceptable_coffee` variable. It should
        be set to true if the coffee has either milk or
        sugar.
    */
    let with_milk: bool = false;
    let with_sugar: bool = true;

    let is_acceptable_coffee: bool = with_milk || with_sugar;
    println!("Is acceptable coffee? {is_acceptable_coffee}.");
    /*
        Declare an array with four i8 integers of your choosing
        Print out the array in its Debug representation.
    */
    let signals_wireless: [i8; 3] = [127, -122, 111];
    println!("🛜 = {signals_wireless:#?}");
    /*
        Declare a tuple consisting of the integer, float,
        a Boolean, and the array that you previously declared.
        Print out the tuple in its Debug representation.
    */
    let climatic_data = (32, 32.532234121, false);
    println!("🌥️ Climatic data = {climatic_data:#?}");
}
