const std = @import("std");

pub fn day(input: []const u8) !void {
    std.debug.print("Running day 1!\n", .{});
    std.debug.print("Input: {s}\n", .{input});

    std.time.sleep(5 * std.time.ns_per_s);
}

test "approximate_timer" {
    var timer = try std.time.Timer.start();

    try day("test_input");

    const end = timer.read() / std.time.ns_per_ms;
    std.testing.expect(end >= 4900 and end <= 5100) catch |err| {
        std.log.err("Expected time was ~5000ms, got: {d}", .{end});
        return err;
    };
}
