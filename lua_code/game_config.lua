local GameConfig = {
    x = 10,
    y = 100,
}

function GameConfig:LoadConfig()
    print("Config Loaded: ", self.x, self.y)
end

function Hello()
    print("Hello game config")
end

return GameConfig