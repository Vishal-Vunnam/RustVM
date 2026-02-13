mod hardware;
mod register; 
mod vm; 

let base_address = f.read_u16::<LittleEndian>().expect("Unable to read the file. ");

let mut address = base_address as usize; 

loop { 
    match f.read_u16::<BigEndian>() { 
        Ok(instruction) => { 
            vm.write_memory(adress, instruction); 
            adress += 1; 
        }
        Err(err) => { 
            if err.kind() == std::io::ErrorKind::UnexpectedEof { 
                break; 
            } else { 
                panic!("Unable to read the file. ");
            }
        }
    } 
}