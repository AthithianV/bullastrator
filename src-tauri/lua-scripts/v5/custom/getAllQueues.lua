-- ARGV[1]: cursor, ARGV[2]: pattern, ARGV[3]: count
local cursor = tonumber(ARGV[1])
local pattern = ARGV[2]
local count = tonumber(ARGV[3])
local scan_result = redis.call('SCAN', cursor, 'MATCH', pattern, 'COUNT', count)
local next_cursor = scan_result[1]
local keys = scan_result[2]
local valid_queues = {}
for _, key in ipairs(keys) do
    local version = redis.call('HGET', key, 'version')
    if version then
        local version_number = string.match(tostring(version), ":(.+)")
        if version_number then
            local major_version = string.match(version_number, "(%d+)")
            if major_version == "5" then
                local queue_name = string.match(key, ":(.+):meta")
                if queue_name then
                    table.insert(valid_queues, queue_name)
                end
            end
        end
    end
end

return {tonumber(next_cursor), valid_queues}