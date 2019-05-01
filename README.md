This is a http server written in Rust, only the standard libraries are used. The purpose of this project is solely for learning. If anyone happens to come across this feel free to contribute your thoughts.

The reason that only the standard libraries are used is that I'm aware that there are better solutions out there in a multitude of languages. I want to take this opportunity to learn the basics of how the more advanced and complete frameworks out there function. I'm not sure if I'm going to create my own sql implementation. I think this might be one of the harder choices I'm going to have to make. I don't have much experience with the various database solutions that are usable. I am probably going to use SQL or Firebase.

__Plans for this Project__ 
- Add proper documentation to the code.
- When the data is being received in the handle_connection method within src/components/connection.rs I should remove the 0 bytes. Currently that is being done later but I think that I should be able to remove an iteration through bytes of incoming data.
- I want to add some form of templating to this project. I'm not sure to what extent but I think that would be helpful.
- I want to add database support, I'm not sure how to due this and I think it will be hard if I want to continue with the no external packages approach.
- I want to create a simple but informative example base page.
- I might want to create a more complicated but properly documented website using this project.

__Instructions for Installing__
- Clone this repo
- run cargo build
- run cargo run

If you want information on any extra commands please run `cargo run help`.