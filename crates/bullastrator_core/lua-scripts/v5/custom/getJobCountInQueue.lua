-- KEYS:
-- [1] = "bull"

-- ARGV:
-- [1] = queue1
-- [2] = queue2
-- ...

local redisPrefix = KEYS[1]
local results = {}

for i = 1, #ARGV do
    local queue = ARGV[i]
    local prefix = redisPrefix .. ":" .. queue

    table.insert(results, {
        queue,
        redis.call("LLEN", prefix .. ":wait"),
        redis.call("LLEN", prefix .. ":active"),
        redis.call("ZCARD", prefix .. ":completed"),
        redis.call("ZCARD", prefix .. ":failed"),
        redis.call("ZCARD", prefix .. ":delayed"),
        redis.call("LLEN", prefix .. ":paused"),
        redis.call("ZCARD", prefix .. ":prioritized")
    })
end

return results
