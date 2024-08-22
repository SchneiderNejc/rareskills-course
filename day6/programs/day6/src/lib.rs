use anchor_lang::prelude::*;

declare_id!("6jFD32aG6PwaYdLUgswa445xL7Lg3uKsLbApJv52CuNG");

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
