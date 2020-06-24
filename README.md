# rust-exec-lua

Execute lua code in rust, 在 Rust 中调用 lua 代码，使用 rlua

> 注意要把 lua.exe和  编译后的 exe 放在一起(如果使用 cargo run运行，则放在工程根目录即可)。对于 Lua 代码，注意 require 的路径