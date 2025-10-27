# RustCommExperiments

https://JimFawcett.github.io/RustCommExperiments.html  
https://jimfawcett.github.io/CommCompare.html  

Repository holds three synchronous Tcp-based message-passing communication crates with different design strategies
- variable message sizes and buffered transfers
- variable message sizes without buffering
- fixed sized messages with buffering

Status: complete. The variable message sizes with buffered transfers is the configuration I've adopted for future work.

Build Process:  

Clone the repository using:  
  git clone https://github.com/JimFawcett/RustCommExperiments.git  
  
  from the command line, navigate into:  
  - RustComm_VariableSizeMsg/rust_com  
  
Issue the command:  
- cargo run --example test4 --release  

That will build the version used for demonstrations in https://jimfawcett.github.io/CommCompare.html
