use crate::vga_buf::SCREEN;
use crate::{print, println};
use lazy_static::lazy_static;
use pc_keyboard::DecodedKey;

lazy_static! {
    static ref SH: spin::Mutex<Shell> = spin::Mutex::new({
        let mut sh = Shell::new();
        sh
    });
}

pub fn handle_keyboard_interrupt(key: DecodedKey) {
    match key { 
        DecodedKey::Unicode(c) => SH.lock().on_key_pressed(c as u8),
        DecodedKey::RawKey(rk) => {}
    }
} 
/*pub enum comand  {
    cur =[b'c',b'u',b'r',b'_',b'd',b'i',b'r',b'\0',b'\0',b'\0'],
    make =[b'm',b'a',b'k',b'e',b'_',b'd',b'i',b'r',b'\0',b'\0'],
    change =[b'c',b'h',b'a',b'n',b'g',b'e',b'_',b'd',b'i',b'r'],
    remove=[b'r',b'e',b'm',b'u',b'v',b'e',b'_',b'd',b'i',b'r'],
    tree =[b't',b'r',b'e',b'e',b'_',b'd',b'i',b'r',b'\0',b'\0'],
    clear =[b'c',b'l',b'e',b'a',b'r',b'\0',b'\0',b'\0',b'\0',b'\0']
}*/
struct Shell {
    buf: [u8; 80],
    buf_len: usize,
    dirs_list: Dirs,
    curr_dirr: Dir
}
struct Dirs {
    dirs: [Dir; 20],
    next_dir: u8,
    is_add:bool
 }
 #[derive(Debug, Clone, Copy)]
 struct Dir {
    index: usize,
    name: [u8; 10],
    parent_index: usize,
    child_count: u8,
    child_indexes: [usize; 10]
 }
 pub fn split(arr:[u8;80])->([u8;10],[u8;10]){
    let mut comand : [u8;10] =[b'\0';10];
    let mut arg : [u8;10] =[b'\0';10]; 
    let mut i : usize=0;
    while arr[i]!=b' ' &&arr[i]!=b'\0' {
        comand[i]=arr[i];        
        i+=1;
        if i==10{
           
            break;
        }
    }
    let mut j: usize=i+1;
    i=0;
    while arr[j+i]!=b'\0' {
        arg[i]=arr[i+j];
        i+=1;
        if i==10{break;}

    }
    return (comand,arg);
 }
pub fn compare_comand(this:[u8;10],that:[u8;10])->(bool){
    let mut is_true=true;
    for i in 0..10  {
        if (this[i]!=that[i]){
            is_true=false;
            break;
        }
    } 
    return is_true;
}

impl Shell {
    pub fn cur_dir(&mut self){
        println!();
        for i in self.curr_dirr.name {
            if(i==b'\0'){break;}
            print!("{}",i as char);
        }
    }
    pub fn suces(&mut self){
        let mut eror = [b's',b'u',b'c',b'e',b's'];
        let mut i=0;
        println!();
        for i in eror {
            print!("{}", i as char);
        }
        println!();
    }
    pub fn make_dir(&mut self,name:[u8;10]){
        if  self.dirs_list.next_dir!=21{
            if (self.curr_dirr.child_count==10){
                let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'f',b'u',b'l',b'l',b' ',b'd',b'i',b'r'];
                let mut i=0;
                println!();
                for i in eror {
                    print!("{}", i as char);
                }
                println!();
            }
            else{
                self.curr_dirr.child_indexes[self.curr_dirr.child_count as usize]=self.dirs_list.next_dir as usize;
                self.curr_dirr.child_count+=1;
                self.dirs_list.dirs[1];
                self.dirs_list.dirs[self.dirs_list.next_dir as usize].index=self.dirs_list.next_dir as usize;
                self.dirs_list.dirs[self.dirs_list.next_dir as usize].name=name;
                
                let mut istrue   = true; 
                for i in 0..20 {
                    if(self.dirs_list.dirs[i].index==21){
                        istrue=false;
                        self.dirs_list.next_dir=i as u8;
                    }
                }
                if istrue {
                    self.dirs_list.next_dir=21;
                }
                else{
                    self.suces();
                }
            }
        }
        else {
            let mut istrue   = true;
            for i in 0..20 {
                if(self.dirs_list.dirs[i].index==21){
                    istrue=false;
                    self.dirs_list.next_dir=i as u8;
                    self.make_dir(name);
                }
            }
            if istrue {
                let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'f',b'u',b'l',b'l',b' ',b'd',b'i',b'r'
                ,b' ',b'l',b'i',b's',b't'];
                let mut i=0;
                println!();
                for i in eror {
                    print!("{}", i as char);
                }
                println!();
            }
        }
    }
    pub fn change_dir(&mut self,name:[u8;10]){
            let mut name2 : [u8;10] =[b'\0';10];
            name2[0]=b'.'; 
        if compare_comand(name, name2){
            if( self.curr_dirr.parent_index!=21){
                self.curr_dirr=self.dirs_list.dirs[self.curr_dirr.parent_index];
                self.suces();
            }
            else {
                let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'i',b's',b' ',b'r',b'o',b'o',b't'];
            println!();
            for i in eror {
                print!("{}", i as char);
            }
            println!();
            }
        } 
        else {
            let mut istrue=true;
            for i in self.curr_dirr.child_indexes{
                if i!=21&&compare_comand(self.dirs_list.dirs[i].name, name){
                    self.curr_dirr=self.dirs_list.dirs[i];
                    istrue=false;
                }
            }
            if istrue{
                let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'n',b'o',b't',b'h',b'i',b'n',b'k',b' ',
            b'n',b'a',b'm',b'e'];
            println!();
            for i in eror {
                print!("{}", i as char);
            }
            println!();
            }
            else {
                self.suces();
            }
        }
    }
    pub fn remove_dir(&mut self,name:[u8;10]){
            let mut istrue=true;
            for i in self.curr_dirr.child_indexes{
                if i!=21&&compare_comand(self.dirs_list.dirs[i].name, name){
                    self.dirs_list.dirs[i as usize].index=21;
                    self.dirs_list.dirs[i as usize].parent_index=21;
                    self.dirs_list.dirs[i as usize].child_count=0;
                    self.dirs_list.dirs[i as usize].child_indexes.fill(21);
                    istrue=false;
                }
            }
            if istrue{
                let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'n',b'o',b't',b'h',b'i',b'n',b'k',b' ',
            b'n',b'a',b'm',b'e'];
            println!();
            for i in eror {
                print!("{}", i as char);
            }
            println!();
            }
            else {
                self.suces();
            }
    }
    fn tree(&mut self, curr_directory: Dir, tab_count: usize) {
        println!();
        for i in 0..curr_directory.child_count {
            let child_directory =self.dirs_list.dirs[curr_directory.child_indexes[i as usize]];
            for tc in 0..tab_count {
                for ts in 0..4 {
                    print!(" ");
                }
            }
            print!(
                "/{}",
                core::str::from_utf8(&child_directory.name)
                    .unwrap()
                    .trim_matches('\0')
            );
            self.tree(child_directory, tab_count + 1);
        }
    }
    pub fn new() -> Shell {
        Shell {
            buf: [0;80],
            buf_len: 0,
            dirs_list: Dirs { dirs: ([Dir{index:21,name:[b'\0';10],parent_index:0,child_count:0,child_indexes:[21;10]};20])
            , next_dir: 1,is_add:true },
            curr_dirr: Dir{index:0,name:[b'r',b'o',b'o',b't',b'\0',b'\0',b'\0',b'\0',b'\0',b'\0'],
            parent_index:21,child_count:0,child_indexes:[21;10],
            
            }
        }
    }
    pub fn on_key_pressed(&mut self, key: u8) {
        if(self.dirs_list.is_add){
            self.dirs_list.dirs[0]=self.curr_dirr;
            self.dirs_list.is_add=false;
        }
        match key {
            b'\n' => {
                let  arg =split(self.buf);
/*                println!();
                for i in arg.1 {
                    print!("{}",(i as char));
                } */
                self.buf.fill(b'\0');
                self.buf_len=0;
                if (compare_comand(arg.0, [b'c',b'u',b'r',b'_',b'd',b'i',b'r',b'\0',b'\0',b'\0'])){
                    self.cur_dir();
                }
                else if  (compare_comand(arg.0,[b'm',b'a',b'k',b'e',b'_',b'd',b'i',b'r',b'\0',b'\0'])){
                    self.make_dir(arg.1);
                }
                else if  (compare_comand(arg.0,[b'c',b'h',b'a',b'n',b'g',b'e',b'\0',b'\0',b'\0',b'\0'])){
                    self.change_dir(arg.1);
                }
                else if  (compare_comand(arg.0,[b'r',b'e',b'm',b'o',b'v',b'e',b'\0',b'\0',b'\0',b'\0'])){
                    self.remove_dir(arg.1);
                }
                else if  (compare_comand(arg.0,[b't',b'r',b'e',b'e',b'_',b'd',b'i',b'r',b'\0',b'\0'])){
                    self.tree(self.curr_dirr, 0);
                }
                else if(compare_comand(arg.0,[b'c',b'l',b'e',b'a',b'r',b'\0',b'\0',b'\0',b'\0',b'\0'])){
                    SCREEN.lock().clear();
                }
                else {
                    let mut eror = [b'[',b'E',b'r',b'r',b'o',b'r',b']',b' ',b'C',b'o',b'm',b'a',b'n',b'd',b' ',b'"'];
                let mut i=0;
                println!();
                for i in eror {
                    print!("{}", i as char);
                }
                for i in arg.0 {
                    if i==b'\0' {break;}
                    print!("{}", i as char);
                }
                eror = [b'"',b'i',b's',b' ',b'n',b'o',b't',b' ',b's',b'u',b'p',b'o',b'r',b't',b'e',b'd'];
                for i in eror {
                    print!("{}", i as char);
                }
                println!();
            }
            }
            08=>{
            }
            _ => {
                self.buf[self.buf_len] = key;
                self.buf_len += 1;
                print!("{}", key as char);
            }
        }
    }
}
