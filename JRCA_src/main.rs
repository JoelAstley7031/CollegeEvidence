extern crate hex;
extern crate byteorder;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::io::Seek;
use std::io::SeekFrom;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::borrow;
use std::borrow::ToOwned;
use std::mem;
use byteorder::{LittleEndian,WriteBytesExt};

fn main() -> io::Result<()> {
    let mut ifilepath = String::new();
    let mut ofilepath = String::new();
    println!("Input file path: ");
    //let f1 = std::io::stdin().read_line(&mut ifilepath).unwrap();
    let f1 = std::io::stdin().read_line(&mut ifilepath).expect("f1 input error"); //Get user input.
    println!("Output file path: ");
    //let f2 = std::io::stdin().read_line(&mut ofilepath).unwrap();
    let f2 = std::io::stdin().read_line(&mut ofilepath).expect("f2 input error"); //Get user input.
    println!("Input Path: {}",ifilepath.trim());
    println!("Output Path: {}",ofilepath.trim());
    let mut ifilepath2 =  ifilepath.to_owned(); //Clone input file path variable for use later.
    let mut ofilepath2 =  ofilepath.to_owned(); //Clone output file path variable for use later.
    let mut infile = File::open(ifilepath.trim())?; //Open input file from input path (trim trailing new line) with read-only permissions.
    let mut outfile = OpenOptions::new().append(true).write(true).open(ofilepath.trim()).unwrap(); //Open output file from output path (trim trailing new line) with write permissions, but without erasing existing file.
    let testkeyhex = "0111"; //Temporary key to use instead of generating key. Not final and will be deprecated.
    const testkeylen: i16 = 2; //Length of temporary key. Not final and will be obsoleted.
    //let byteonehex = "01";
    let testkey = hex::decode(testkeyhex).expect("Decode testkeyhex error."); //Convert hex string to raw bytes.
    //let byteone = hex::decode(byteonehex).expect("Decode byteonehex error.");
    println!("test key: {:?}",testkey);
    let mut infilebuf2 = [0;2]; //2-byte buffer for reading infile.
    let mut infilebuf1 = [0;1]; //1-byte buffer for reading infile.
    let mut infilepos = 1; //Current position in input file stream.
    let mut infileread2 = true; //Did we just read 2 bytes? false if we only read 1 byte.
    let mut infilerepeat = false; //Are we reading consecutive bytes?
    let mut infilercount = 1i16; //How many of the byte have we counted in a row? 1 unless consecutive. Use i16 type.
    let mut ofilebuf1 = [0;1]; //1-byte buffer for writing a single byte to outfile.
    const ofilebuf2size: i16 = testkeylen + 1; //Size of ofilebuf2 will be the length of our key (2 for test key), plus 1 for the byte we are compressing.
    let mut ofilebuf3 = [0;2]; //2-byte buffer for storing the total number of consecutive bytes we are currently compressing. Little Endian, i16.
    let mut ofilebuf2 = [0;ofilebuf2size as usize];
    let mut ofilebuf2kcount = testkeylen + 1; //How many bytes of our key have we copied to ofilebuf2 so far?
    let mut ofilebuf4 = [0;1]; //1-byte buffer for copying a single byte from infile to outfile.
    let infilemeta = fs::metadata(ifilepath2.trim())?; //Get file metadata so we can get size of file in bytes.
    let infilelen = infilemeta.len(); //Get size of file in bytes from the file metadata.
    let mut infilelenm1 = infilelen - 1; //Possibly unneccesary.
    println!("Length of infile: {}",infilelen);
    println!("Attempting to read infile");
    while infilepos <= infilelen {
        //Check to ensure that there are 2 unread bytes left in the filestream. If not, just read 1 byte.
        if infilepos == infilelen { //There is only 1 unread byte left, don't attempt to read 2 like normal.
            let infileread = infile.read(&mut infilebuf1[..])?; //Read 1 byte into infilebuf1.
            infilepos = infilepos + 1;
            infileread2 = false;
            println!("Reading 1 byte from offset {}, result is {:?}.",infilepos,&infilebuf1[..infileread]);
            //There is only one byte left, just write it to outfile.
            ofilebuf1[0] = infilebuf1[0];
            outfile.write(&ofilebuf1)?;
            infile.read(&mut infilebuf1[..]);
            //println!("Written final byte!")
        } else { //There are at least 2 unread bytes left, so we are fine to read 2 bytes like we normally do.
            let infileread = infile.read(&mut infilebuf2[..])?; //Read 2 bytes into infilebuf2.
            infile.seek(SeekFrom::Start(infilepos))?; //Set stream cursor as if we had only read a single byte. Allows us to compare the byte we just read with the byte that we read next.
            infilepos = infilepos + 1;
            //infileread2 = true;
            println!("Reading 2 bytes from offset {}, result is {:?}.",infilepos,&infilebuf2[..infileread]);
            //println!("{}",infilerepeat);
            //println!("Testing if these 2 bytes are consecutive.");
            if infilebuf2[1] == infilebuf2[0] { //Is the byte we just read identical to the previous byte?
                //Bytes are consecutive.
                //println!("Bytes are consecutive.");
                infilerepeat = true; //Set to true so that we stop writing the bytes we read to outfile.
                infilercount = infilercount + 1; //Keep track of how many times the byte is repeated in a row.
                //println!("Byte repeat count: {}",infilercount);
            } else {
                //Bytes are not consecutive. If we were already reading consecutive bytes, compress them and then write the byte we just read after. Otherwise write only the byte we just read.
                if infilerepeat == true {
                    //Compress consecutive bytes.
                    //We want to overwrite the last two bytes we wrote to outfile, otherwise the compression will not compress all of the consecutive bytes.
                    ofilebuf1[0] = infilebuf2[1]; //Copy the byte that we just read to a buffer for writing after we have written the compressed bytes.
                    let ofilecompsize: i16 = ofilebuf2size + 2;
                    
                    if infilercount > ofilecompsize {
                        for (i,keybyte) in testkey.iter().enumerate() {
                            //println!("{}",i);
                            ofilebuf2[i] = *keybyte;
                            //println!("Copying testkey[{}] to ofilebuf2[{}]",i,i+1);
                        }
                        //println!("{}",ofilebuf2kcount);
                        ofilebuf2[(ofilebuf2kcount as usize)-1] = infilebuf2[0]; //Append the consecutive byte to the end of ofilebuf2.
                        ofilebuf3 = [0;2]; //2-Byte buffer for writing the value of infilercount to outfile.
                        ofilebuf3.as_mut().write_i16::<LittleEndian>(infilercount - 1).expect("Unable to copy infilercount to ofilebuf3."); //Copy infilercount's value to a ofilebuf3 for writing.
                        //We now start writing the compressed data to outfile.
                        //println!("Writing compressed data to outfile");
                        //outfile.seek(SeekFrom::Current(-1))?;
                        outfile.write(&ofilebuf2)?; //Write the contents of ofilebuf2 to outfile.
                        outfile.write(&ofilebuf3)?; //Write the contents of ofilebuf3 to outfile.
                        //Compressed data has been written, now we write the byte we copied to ofilebuf1 after our compressed data.
                        //outfile.write(&ofilebuf1)?; //Write byte after compressed data.
                        //println!("Compressed {} bytes into {} bytes!",infilercount-2,ofilebuf2kcount);
                        infilerepeat = false; //Reset variables.
                        infilercount = 1; //Reset variables.
                    } else {
                        ofilebuf1[0] = infilebuf2[1]; //Copy the byte that we just read to a buffer for writing.
                        ofilebuf4[0] = infilebuf2[0]; //Copy our consecutive byte to a buffer for writing.
                        let mut ofilewritecount = 1i16;
                        while ofilewritecount < (infilercount + 1) { //Write our consecutive bytes straight to outfile without compressing them, because it would have no positive impact on file size.
                            outfile.write(&ofilebuf4)?; //Write consecutive byte.
                            ofilewritecount = ofilewritecount + 1;
                            //println!("Written consecutive byte in uncompressed form due to compression being unable to save any bytes.");
                        }
                        //outfile.write(&ofilebuf1)?; //Write byte after consecutive bytes.
                        infilerepeat = false; //Reset variables.
                        infilercount = 1; //Reset variables.
                    }
                } else {
                    if infilepos >= 2 {
                        if infilerepeat == false {
                            ofilebuf4[0] = infilebuf2[0];
                            outfile.write(&ofilebuf4)?; //Copy the first byte we read because this is the start of the file.
                        }
                    }
                    //ofilebuf4[0] = infilebuf2[1];
                    //if infilerepeat == false {
                    //    if infilepos > 2 {
                    //        ofilebuf4[0] = infilebuf2[1];
                    //        outfile.write(&ofilebuf4)?; //Copy the byte we just read from infile to outfile.
                    //    }
                    //}
                    //infilerepeat = false;
                    //infilercount = 1;
                }
            }
        }
    }
    if infilerepeat == true {
        //let mut ofilewritecount = 1i16;
        //ofilebuf1[0] = infilebuf1[0];
        //while ofilewritecount < (infilercount) { //Write our consecutive bytes straight to outfile without compressing them, because it would have no positive impact on file size.
        //    outfile.write(&ofilebuf1)?; //Write consecutive byte.
        //    ofilewritecount = ofilewritecount + 1;
        //    //println!("Written consecutive byte in uncompressed form due to compression being unable to save any bytes.");
        //}
        let ofilecompsize: i16 = ofilebuf2size + 2;
        
        //if infilercount > ofilecompsize {
            for (i,keybyte) in testkey.iter().enumerate() {
                //println!("{}",i);
                ofilebuf2[i] = *keybyte;
                //println!("Copying testkey[{}] to ofilebuf2[{}]",i,i+1);
            }
            //println!("{}",ofilebuf2kcount);
            ofilebuf2[(ofilebuf2kcount as usize)-1] = infilebuf2[0]; //Append the consecutive byte to the end of ofilebuf2.
            ofilebuf3 = [0;2]; //2-Byte buffer for writing the value of infilercount to outfile.
            ofilebuf3.as_mut().write_i16::<LittleEndian>(infilercount - 1).expect("Unable to copy infilercount to ofilebuf3."); //Copy infilercount's value to a ofilebuf3 for writing.
            //We now start writing the compressed data to outfile.
            //println!("Writing compressed data to outfile");
            //outfile.seek(SeekFrom::Current(-1))?;
            outfile.write(&ofilebuf2)?; //Write the contents of ofilebuf2 to outfile.
            outfile.write(&ofilebuf3)?; //Write the contents of ofilebuf3 to outfile.
        //}
    }
    println!("Reading infile finished. Read {} bytes out of {}.",infilepos,infilelen);
    let mut f3s = String::new();
    let f3 = std::io::stdin().read_line(&mut f3s).expect("f3s input error"); //Get user input.
    Ok(()) //End program.
}
