mod lua_exec;
extern crate rlua;
fn main() -> Result<(),rlua::Error> {
    let code_path = "./lua_code/main.lua";
    lua_exec::exec(code_path)?;

    Ok(())
}
