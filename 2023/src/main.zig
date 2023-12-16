const std = @import("std");
const mem = std.mem;
const Allocator = mem.Allocator;

fn handle_day(day: usize) !void {
    return switch (day) {
        1 => @import("days/1.zig").day(@embedFile("inputs/1.txt")),
        else => if (day <= 0 or day > 25) error.OutsideOfRange else error.NoDay,
    };
}

fn run_day(day: usize) !void {
    var time = try std.time.Timer.start();
    try handle_day(day);

    const timer_f: f64 = @floatFromInt(time.read());
    const ns_per_ms_f: f64 = @floatFromInt(std.time.ns_per_ms);
    const elapsed_ms: f64 = timer_f / ns_per_ms_f;
    std.debug.print("Time to run day {d}: {d}ms\n\n", .{ day, elapsed_ms });
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const alloc = arena.allocator();

    const args = try std.process.argsAlloc(alloc);
    defer std.process.argsFree(alloc, args);

    if (args.len >= 2) { // Running a single day
        const day = try std.fmt.parseInt(usize, args[1], 10);
        run_day(day) catch |err| {
            switch (err) {
                error.OutsideOfRange => std.debug.print("Psssssh AoC only has puzzles for 1-25\n", .{}),
                error.NoDay => std.debug.print("No code for day {d} :(\n", .{day}),
                else => return err,
            }
        };
    } else { // Running all days
        std.debug.print("Running all days!\n\n", .{});
        // Loop through and run all the implemented days
        for (1..26) |day| {
            run_day(day) catch |err| {
                // Don't throw an error on NoDay since this is fine
                if (err != error.NoDay) {
                    return err;
                }
            };
        }
    }
}
