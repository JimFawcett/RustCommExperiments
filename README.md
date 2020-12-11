# RustCommExperiments

https://JimFawcett.github.io/RustCommExperiments.html

Repository holds three synchronous Tcp-based message-passing communication crates with different design strategies
- variable message sizes and buffered transfers
- variable message sizes without buffering
- fixed sized messages with buffering

Status: complete. The variable message sizes with buffered transfers is the configuration I've adopted for future work.

Build Process:<hr />
Clone the repository using:<hr />
  git clone https://github.com/JimFawcett/RustCommExperiments.git<hr />
  from the command line, navigate into:<hr />
    RustComm_VariableSizeMsg/rust_com<hr />
Issue the command:<hr />
  cargo run --example test4 --release<hr />

That will build the version used for demonstrations in https://jimfawcett.github.io/CommCompare.html
