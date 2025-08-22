# Woman

Personalised man pages for my custom note taking


## Why

I have a terrible memory, so I thought I'd make a tool to help me take notes about all the different tools and common pitfalls / uses


## Outline / Design

`woman <appname>` should open up a reader screen much like `man` does
If it's piped into something / or `-p` is passed it should just print raw to the terminal

`woman -e <appname>` or any ordering including `-e` should open up the prefered editor with any currently written parts and then save whatever is written to the file to the db

I'd like for there to be consistent headers in the file for certian sections, whilst I don't mind there being a lot of more use-case specific headers I'd like the headers:
  - "Common Uses" : examples of command line arguments / quick common usecases
  - "TLDR" : what does the app do, really should be 1 or two sentences
  - "Resources" : links that might be useful


## Things to look at

[inquire](https://lib.rs/crates/inquire) : for taking user input 
[clap](https://docs.rs/clap/latest/clap/) : for command line arguments
[comfy-table](https://lib.rs/crates/comfy-table) : nice looking output tables
[atty](https://crates.io/crates/atty) : am I a tty (this is what I want)
