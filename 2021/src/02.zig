const std = @import("std");
const stringToEnum = std.meta.stringToEnum;
const parseInt = std.fmt.parseInt;
const mem = std.mem;
const log = std.log;
const print = std.debug.print;

const Action = enum {
    forward,
    up,
    down,
};

pub fn day02_1(input: []const u8) !u32 {
    var iter = mem.split(input, "\n");
    var depth: u32 = 0;
    var horizontal: u32 = 0;

    while (iter.next()) |line| {
        // split into two items
        var inner_iter = mem.split(line, " ");

        // skip laste entry
        if (line.len == 0) {
            break;
        }

        // grab first item, parse into enum
        const action = inner_iter.next().?;
        const action_enum = stringToEnum(Action, action).?;

        // grab second item, parse into u32
        const parse_value = inner_iter.next().?;
        const value = try parseInt(u32, parse_value, 10);

        // follow action
        switch (action_enum) {
            Action.forward => horizontal += value,
            Action.up => depth -= value,
            Action.down => depth += value,
        }
    }

    // multiple and return
    const result = depth * horizontal;
    return result;
}

test "day02 problem 1" {
    const input = @embedFile("../input/02.txt");

    const ret = try day02_1(input);

    try std.testing.expect(ret == 2120749);
}
