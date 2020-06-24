local GameConfig = require('lua_code.game_config')
local Human = require('lua_code.actors.human')
local Monster = require('lua_code.actors.monster')
print("This is lua code")
GameConfig:LoadConfig()
Hello() -- 这个方法是GameConfig中的，如果有另一个脚本里也有Hello，后require的将覆盖前require的
Human:Hello()
Monster:Hello()