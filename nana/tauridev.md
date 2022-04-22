
## Tauri Source Code Notes

#### How does everything get kicked off

looks like everything gets kicked off in **app.rs**  
search for pub fn run  
and check out the method pub fn build

search for default() in app.rs for more details   
see on_event_loop_event()   


#### Codegen

when you search for *codegen* you will note that it is used in the following 2 crates

* tauri-build
* tauri-macros
