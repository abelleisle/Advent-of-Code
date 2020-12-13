waypoint = {
    offset = {x = 10, y = 1}
}
ferry = {
    pos = {x = 0, y = 0},
}

for line in io.lines("../input.txt") do
    if type(line) ~= nil then
        ins = line:sub(1,1)
        cnt = tonumber(line:sub(2))
        print('Dir: '.. ins .. ' Cnt: '.. cnt)

        if     ins == "N" then
            waypoint.offset.y = waypoint.offset.y + cnt
        elseif ins == "E" then
            waypoint.offset.x = waypoint.offset.x + cnt
        elseif ins == "S" then
            waypoint.offset.y = waypoint.offset.y - cnt
        elseif ins == "W" then
            waypoint.offset.x = waypoint.offset.x - cnt
        elseif ins == "R" or ins == "L" then
            local angle = 0
            if ins == "R" then
                angle = -cnt;
            elseif ins == "L" then
                angle = cnt;
            end
            angle = angle * (math.pi/180);
            x = waypoint.offset.x;
            y = waypoint.offset.y;
            waypoint.offset.x = x * math.cos(angle) - y * math.sin(angle)
            waypoint.offset.y = x * math.sin(angle) + y * math.cos(angle)
        elseif ins == "F" then
            ferry.pos.x = ferry.pos.x + (waypoint.offset.x * cnt)
            ferry.pos.y = ferry.pos.y + (waypoint.offset.y * cnt)
        end
    end
end

print('Final Position: '..ferry.pos.x..','..ferry.pos.y)
print('Final Manhattan: '..math.abs(ferry.pos.x)+math.abs(ferry.pos.y))
