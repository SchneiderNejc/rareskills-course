use anchor_lang::prelude::*;

// *** CONSTANT DECLARED HERE ***
const MEANING_OF_LIFE_AND_EXISTENCE: u64 = 42;

declare_id!("6UBGbvb6AysTziHTszALb4ghqWdCDSNCwUkyNH7qYHiV");

#[program]
pub mod day6 {
    use super::*;
    // Import HashMap library for mappings
    use std::collections::HashMap;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        for i in (0..10).step_by(2) {
            // do something...

            msg!("{}", i);
        }
        Ok(())
    }


    pub fn fixed_array(ctx: Context<Initialize>) -> Result<()> {
        // Declare an array of u32 with a fixed size of 5
        let my_array: [u32; 5] = [10, 20, 30, 40, 50];

        // Accessing elements of the array
        let first_element = my_array[0];
        let third_element = my_array[2];

        // Declare a mutable array of u32 with a fixed size of 3
        let mut mutable_array: [u32; 3] = [100, 200, 300];

        // Change the second element from 200 to 250
        mutable_array[1] = 250;

        // Rest of your program's logic

        Ok(())
    }

    pub fn dynamic_array(ctx: Context<Initialize>) -> Result<()> {
        // Declare a dynamic array-like structure using Vec
        let mut dynamic_array: Vec<u32> = Vec::new();

        // Add elements to the dynamic array
        dynamic_array.push(10);
        dynamic_array.push(20);
        dynamic_array.push(30);

        // Accessing elements of the dynamic array
        let first_element = dynamic_array[0];
        let third_element = dynamic_array[2];

        // Rest of your program's logic
        msg!("Third element = {}", third_element);

        Ok(())
    }

    pub fn mapping(ctx: Context<Initialize>, key: String, value: String) -> Result<()> {
        // Initialize the mapping
        let mut my_map = HashMap::new();

        // Add a key-value pair to the mapping
        my_map.insert(key.to_string(), value.to_string());

        // Log the value corresponding to a key from the mapping
        msg!("My name is {}", my_map[&key]);

        Ok(())
    }

    // Note: struct is reserved keyword, therefore we use structure
    pub fn structure(_ctx: Context<Initialize>, name: String, age: u64) -> Result<()> {
        // Defining a struct in Solana
        struct Person {
            my_name: String,
            my_age: u64,
        }

        // Creating an instance of the struct
        let mut person1: Person = Person {
            my_name: name,
            my_age: age,
        };

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        // Accessing and modifying struct fields
        person1.my_name = "Bob".to_string();
        person1.my_age = 18;

        msg!("{} is {} years old", person1.my_name, person1.my_age);

        Ok(())
    }

    pub fn life_meaning(ctx: Context<Initialize>) -> Result<()> {
        msg!(&format!("Answer to the ultimate question: {}", MEANING_OF_LIFE_AND_EXISTENCE)); // new line here
        Ok(())
    }

    pub fn cast_array_length(ctx: Context<Initialize>) -> Result<()> {

        let mut dynamic_array: Vec<u32> = Vec::from([1,2,3,4,5,6]);
        let len = dynamic_array.len(); // this has type usize

        let another_var: u64 = 5; // this has type u64

        let len_plus_another_var = len as u64 + another_var;

        msg!("The result is {}", len_plus_another_var);

        Ok(())
     }

     pub fn filter_even_numbers(ctx: Context<Initialize>,
        input_vector: Vec<u64>) -> Result<()> {

        let mut even_numbers = Vec::new();

        for number in input_vector.iter() {
            if number % 2 == 0 {
                even_numbers.push(number);
            }
        }

        msg!("Even numbers: {:?}", even_numbers);
        Ok(())
     }


    pub fn age_checker(ctx: Context<Initialize>,
        age: u64) -> Result<()> {

        match age {
            1 => {
                // Code block executed if age equals 1
                msg!("The age is 1");
            },
            2 | 3 => {
                // Code block executed if age equals 2 or 3
                msg!("The age is either 2 or 3");
            },
            4..=6 => {
                // Code block executed if age is in the
                // range 4 to 6 (inclusive)
                msg!("The age is between 4 and 6");
            },
            _ => {
                // Code block executed for any other age
                msg!("The age is something else");
            }
        }
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Initialize {}
