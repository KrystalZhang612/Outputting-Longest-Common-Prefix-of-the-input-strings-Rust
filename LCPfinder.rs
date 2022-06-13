
//use Option<T> to enum two variants

fn find_the_longest_common_prefix_amongst_all(given_string_array: &[&[u8]]) -> Option<Vec<u8>> {
    
    //if the given array of strings do not have any common prefix, 
    //return the empty string as the base case
    
    if given_string_array.is_empty() {
        
        
        return None;
        
    }
    
    //create a new empty vector in mutable type 
    
    let mut vec = Vec::new();
    
    //create a iterator for traversal in looping the given array of strings 
    
    let mut counter = 0;
    
    //as same as for loop
    //to append CP characters in iterable list of strings for CP
    
    loop {
        
        
        let mut current_character = None;
    
        //if there is LCP in the gievn array of strings 
        
        for LCP_amongst_all in given_string_array {
            
            //if the length of the LCP string is the same as the counter index position
            //wrap extract the new string vector value as the LCP string using Some() 
            
            if counter  == LCP_amongst_all.len() {
                
                return Some(vec);
            }
            
            
            //match the current character with the rest of characters in the string list 
            
            
            match current_character{
                
                None => {
                    
                    current_character = Some(LCP_amongst_all[counter]);
                    
                }
                
                //traverse the rest characters in the original given array of strings
                //if other characters do not match the characters within LCP, continue
                
                
                Some(other_characters) if other_characters != LCP_amongst_all[counter] => return Some(vec),
                
                _ => continue,
            }
        }
        
        //if there is other characters matches 
        //recursively append it as CP characters to the result new vec as the ultimate LCP while traversalling
        
        if let Some(other_characters) = current_character {
            
            vec.push(other_characters);
            
        }
        
        //keep traversalling through the entire string list 
        
        counter += 1;
    }
    
}

//driver 

fn main() {
    
    let strs: [&[&[u8]]; 1] = [
        
        &[b"apple", b"app", b"aple", b"appl"],
    ];
    
    strs.iter().for_each(|given_string_array| match find_the_longest_common_prefix_amongst_all(given_string_array) {
        
      
        Some(LCP_result) => println!("{}", String::from_utf8_lossy(&LCP_result)),
        
        
        None => println!(),
        
    });
}
