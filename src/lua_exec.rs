
use std::io::Read;
use std::io::BufReader;
use std::fs::File;
use rlua::Lua;
use std::path::Path;

fn load_code(str_path: &str) -> Vec<u8> {
    let file_path = Path::new(str_path);
    let f = File::open(&file_path).unwrap();
    let mut reader = BufReader::new(f);
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).unwrap();
    //let s = ::std::str::from_utf8(&buf).unwrap();
    buf
}


pub fn exec(lua_file: &str) -> Result<(), rlua::Error> {
    let lua = Lua::new();
    let monster_vec = load_code(lua_file);
    let monster_code = std::str::from_utf8(&monster_vec).unwrap();

    lua.context(|lua_context| {
        lua_context.load(monster_code).exec()
    })?;

    Ok(())
}