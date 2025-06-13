const std = @import("std");

const cli = @import("cli.zig");

pub fn main() !void {
    var stdout_ = std.io.getStdOut();
    var stdout = stdout_.writer();

    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const ma = gpa.allocator();

    var options = cli.Options{};
    try options.init(ma);
    defer options.deinit();

    try stdout.print("Everything went OK\n", .{});
}
