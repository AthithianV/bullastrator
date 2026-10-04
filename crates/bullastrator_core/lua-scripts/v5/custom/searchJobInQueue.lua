-- KEYS[1]: The list or zset key (e.g., "bull:myqueue:active")
-- ARGV[1]: "1" if zset, "0" if list
-- ARGV[2]: Start index
-- ARGV[3]: Stop index
-- ARGV[4]: Job key prefix (e.g., "bull:myqueue:")
-- ARGV[5]: Data keyword (pass "" to ignore)
-- ARGV[6]: Failed reason keyword (pass "" to ignore)

local is_zset = ARGV[1] == "1"
local start_idx = tonumber(ARGV[2])
local stop_idx = tonumber(ARGV[3])
local job_prefix = ARGV[4]

local data_kw = ARGV[5]
local reason_kw = ARGV[6]

local job_ids
if is_zset then
    -- ZREVRANGE is used for delayed, waiting, completed, failed (sorted by timestamp)
    job_ids = redis.call('ZREVRANGE', KEYS[1], start_idx, stop_idx)
else
    -- LRANGE is used for active, wait (standard lists)
    job_ids = redis.call('LRANGE', KEYS[1], start_idx, stop_idx)
end

local matches = {}
local total_scanned = 0

for i, id in ipairs(job_ids) do
    total_scanned = i
    local job_key = job_prefix .. id
    
    -- Fetch all potentially needed fields at once to minimize internal overhead
    local res = redis.call('HMGET', job_key, 'failedReason', 'data')
    local failed_reason = res[1]
    local data = res[2]

    local is_match = true

    -- 2. Failed Reason Filter
    -- The '1, true' arguments in string.find mean "start at index 1, do a plain string match (no regex)"
    if is_match and reason_kw ~= "" then
        if not failed_reason or not string.find(failed_reason, reason_kw, 1, true) then
            is_match = false
        end
    end

    -- 3. Data Keyword Filter (Most expensive check, done last)
    if is_match and data_kw ~= "" then
        if not data or not string.find(data, data_kw, 1, true) then
            is_match = false
        end
    end

    if is_match then
        table.insert(matches, id)

        if #matches >= 50 then
            break
        end
    end
end

return {matches, total_scanned}