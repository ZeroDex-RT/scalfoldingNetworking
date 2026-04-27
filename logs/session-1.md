# Plan For Tonight  

## Session 1

-- make some progress with the rust project or try to learn rust by doing some random stuff.
-- become comfortable with the nvim and lazy vim ide.
-- watch some cool stuff with raspberry PI (while taking a break).

### Start By 

--- what is the project about 

Project is about learning basics of networking and rust. learn about how to establish TCP connections how to find ports open. We have to find open ports
if they are vulnerable to attacks or not. we can even add AI to check the logs reported by the scanner inorder to establish vulnerability.

---- whats already done 

I have already created a git repository, I have already initialized a project so far with cargo init, I have written some rust code for tcp connections
which scans for open ports on a random website using net::{TcpStream} i think.

I will today establish what I have to do in the project all together. better description.


### Project Description

- Scanner for Open Ports across a newtork.
- Banner Grabbing - which services are running on the open ports.
- CLI for using the tool.
- Better Output as in JSON or some other kind of file.
- Server in which the tool could be used, deploy it for learning how its deployed.



### Learned about

-- how project structure is made in rust. 
  - Usage of mod.rs inside of folders.
  - How to define files inside of mod.rs (pub mod scanner).
  - importing a module you have created - mod network (folder name).
-- learned basics about multithreading.
  - how its used to split work among threads and improve efficiency
  - defined in std::thread library. thread::spawn(move || {work that you want});





