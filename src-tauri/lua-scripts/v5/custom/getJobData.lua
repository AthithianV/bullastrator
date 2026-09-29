--[[
  Bullastrator Job Fetcher

  KEYS[1] -> State key
             e.g. bull:myqueue:wait

  ARGV[1] -> Range command
             LRANGE / ZRANGE / ZREVRANGE

  ARGV[2] -> Count command
             LLEN / ZCARD

  ARGV[3] -> start index
  ARGV[4] -> end index

  ARGV[5] -> Job key prefix
             e.g. bull:myqueue:

  ARGV[6...] -> Optional selected job IDs

  Behavior:
    - If job IDs are provided:
        fetch exactly those jobs
    - If no job IDs:
        fetch jobs using the state key/range
--]]

local state_key = KEYS[1]
local range_cmd = ARGV[1]
local count_cmd = ARGV[2]
local start_idx = tonumber(ARGV[3])
local end_idx = tonumber(ARGV[4])
local prefix = ARGV[5]

local ids = {}
local total_count = 0

-- ============================================================
-- 1. Determine which job IDs to fetch
-- ============================================================

if #ARGV >= 6 then
    -- Explicitly selected jobs
    for i = 6, #ARGV do
        table.insert(ids, ARGV[i])
    end

    total_count = #ids
else
    -- Normal queue pagination
    ids = redis.call(range_cmd, state_key, start_idx, end_idx)
    total_count = redis.call(count_cmd, state_key)
end


-- ============================================================
-- 2. Hydrate jobs
-- ============================================================

local jobs = {}

for _, id in ipairs(ids) do
    local job_key = prefix .. id

    local data = redis.call(
        'HMGET',
        job_key,

        'data'
    )

    if data[1] then
        local job = data[1]

        table.insert(jobs, job)
    end
end


-- ============================================================
-- 3. Return JSON
-- ============================================================

return cjson.encode({
    count = total_count,
    jobs = jobs
})
