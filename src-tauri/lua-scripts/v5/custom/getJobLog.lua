-- KEYS[1]: Job key prefix (e.g. "bull:myqueue:")
-- KEYS[2]: Job ID
-- ARGV[1]: Start index (optional)
-- ARGV[2]: Stop index (optional)

local start_idx = tonumber(ARGV[1]) or 0
local stop_idx = tonumber(ARGV[2]) or -1

local job_prefix = KEYS[1]
local job_id = KEYS[2]

local logs_key = job_prefix .. job_id .. ":logs"

local res = redis.call("LRANGE", logs_key, start_idx, stop_idx)

return res
