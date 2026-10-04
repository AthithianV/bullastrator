--[[
  Simplified Paginated Job Fetcher
  
  KEYS[1] -> The State Key (e.g., "bull:myqueue:active")
  
  ARGV[1] -> The Command (e.g., "LRANGE", "ZRANGE", or "ZREVRANGE")
  ARGV[2] -> The Count Command (e.g., "LLEN" or "ZCARD")
  ARGV[3] -> start_index
  ARGV[4] -> end_index
  ARGV[5] -> job_key_prefix
]]

local state_key = KEYS[1]
local range_cmd = ARGV[1]
local count_cmd = ARGV[2]
local start_idx = tonumber(ARGV[3])
local end_idx = tonumber(ARGV[4])
local prefix = ARGV[5]

-- 1. Fetch IDs and Total Count using the passed commands
local ids = redis.call(range_cmd, state_key, start_idx, end_idx)
local count = redis.call(count_cmd, state_key)

-- 2. Hydrate Jobs
local jobs = {}
for i, id in ipairs(ids) do
    local job_key = prefix .. id
    local data = redis.call('HMGET', job_key, 'name', 'timestamp', 'processedOn', 'delay', 'priority', 'attemptsMade', 'finishedOn')
    
    if data[1] then
        table.insert(jobs, {id, data}) 
    end
end

return {count, jobs}